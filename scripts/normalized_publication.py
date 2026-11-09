"""Prepare an isolated, explicitly transformed publication from audited inputs."""
import json
from pathlib import Path
import subprocess
import tempfile
import tarfile
import tomllib

import registry_package_audit as registry


def prepare(root, audit_directory, archive, target, expected_commit):
    root, audit_directory, target = Path(root).resolve(), Path(audit_directory), Path(target).resolve()
    proof = json.loads((audit_directory / 'registry-package.json').read_text())
    registry.verify(audit_directory, expected_commit, proof['database'], archive)
    target.mkdir(parents=True, exist_ok=True)
    workspace = Path(tempfile.mkdtemp(prefix='normalized-publication-', dir=target))
    checkout = workspace / 'package'
    subprocess.run(['git', 'clone', '--no-local', '--no-checkout', str(root), str(checkout)], check=True)
    subprocess.run(['git', '-C', str(checkout), '-c', 'core.autocrlf=false', '-c', 'core.symlinks=true',
                    'checkout', '--detach', expected_commit], check=True)
    # Removing path dependencies changes Cargo's nested-package exclusions.
    # Materialize exactly the files present in the already audited source
    # archive so the normalized manifest cannot accidentally add other crates.
    def pattern(name):
        for character in ('\\', '*', '?', '[', ']'):
            name = name.replace(character, '\\' + character)
        return '/' + name
    selected = sorted({*proof['source_files_sha256'], 'Cargo.toml'})
    subprocess.run(['git', '-C', str(checkout), 'sparse-checkout', 'set', '--no-cone', '--stdin'],
                   input='\n'.join(pattern(name) for name in selected) + '\n', text=True, check=True)
    # Git may normalize line endings even for a clean source checkout. Preserve
    # the exact already audited archive bytes, never just equivalent Git text.
    with tarfile.open(archive) as packaged:
        for entry in packaged.getmembers():
            relative = entry.name.split('/', 1)[1]
            if relative not in proof['source_files_sha256']:
                continue
            destination = checkout / relative
            if not destination.parent.resolve().is_relative_to(checkout.resolve()):
                raise ValueError('Audited source path escapes the isolated checkout')
            if destination.is_symlink():
                destination.unlink()
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(packaged.extractfile(entry).read())
    # The wrapper prevents this nested project from joining the caller's
    # workspace, without changing one byte of the audited package manifest.
    (workspace / 'Cargo.toml').write_text('[workspace]\nmembers=["package"]\nresolver="3"\n', encoding='utf-8')
    (checkout / 'Cargo.toml').write_bytes((audit_directory / 'registry-package.toml').read_bytes())
    lock = (audit_directory / 'registry-package.lock').read_bytes()
    (workspace / 'Cargo.lock').write_bytes(lock)
    (checkout / 'Cargo.lock').write_bytes(lock)
    metadata = json.loads(subprocess.check_output([
        'cargo', 'metadata', '--locked', '--no-deps', '--format-version', '1',
        '--manifest-path', str(checkout / 'Cargo.toml')], cwd=checkout, text=True))
    if Path(metadata['workspace_root']).resolve() != workspace.resolve():
        raise ValueError('Normalized publication joined an unexpected workspace')
    owned_target = Path(metadata['target_directory'])
    if not owned_target.is_absolute():
        raise ValueError('Normalized publication target is not absolute')
    return checkout, owned_target


def candidate(checkout, target, audit_directory, expected_commit):
    checkout, target, audit_directory = Path(checkout), Path(target), Path(audit_directory)
    package = tomllib.loads((checkout / 'Cargo.toml').read_text(encoding='utf-8'))['package']
    archive = target / 'package/tmp-crate' / f"{package['name']}-{package['version']}.crate"
    if not archive.resolve().is_relative_to(target.resolve()):
        raise ValueError('Publication candidate lies outside its owning target')
    archive.unlink(missing_ok=True)
    # Inspect the actual temporary archive built by publish, rather than an
    # earlier top-level .crate which Cargo does not overwrite on this path.
    subprocess.run(['cargo', 'publish', '--dry-run', '--locked', '--no-verify', '--allow-dirty',
                    '--registry', 'crates-io', '--manifest-path', str(checkout / 'Cargo.toml')],
                   cwd=checkout, check=True)
    proof = json.loads((audit_directory / 'registry-package.json').read_text())
    registry.verify(audit_directory, expected_commit, proof['database'], archive,
                    normalized_publication=True)
    return archive


def record_transform(audit_directory, candidate_archive, expected_commit, destination, *, published=False):
    audit_directory = Path(audit_directory)
    proof = json.loads((audit_directory / 'registry-package.json').read_text())
    registry.verify(audit_directory, expected_commit, proof['database'], candidate_archive,
                    normalized_publication=True)
    with tarfile.open(candidate_archive) as packaged:
        vcs_entry = next(entry for entry in packaged.getmembers() if entry.name.endswith('/.cargo_vcs_info.json'))
        vcs = json.load(packaged.extractfile(vcs_entry))['git']
    record = {'schema': 1, 'kind': 'normalized-manifest-and-registry-lock',
              'product_commit': expected_commit, 'vcs_dirty': bool(vcs.get('dirty')),
              'candidate_vcs_commit': vcs['sha1'],
              'dependency_audit_sha256': registry.sha((audit_directory / 'provenance.json').read_bytes()),
              'registry_package_sha256': registry.sha((audit_directory / 'registry-package.json').read_bytes()),
              'candidate_archive_sha256': registry.sha(Path(candidate_archive).read_bytes()),
              'manifest_sha256': proof['manifest_sha256'], 'lock_sha256': proof['lock_sha256'],
              'source_files_sha256': proof['source_files_sha256']}
    record['artifact_stage'] = 'downloaded-registry-archive' if published else 'pre-upload-candidate'
    if published:
        record['published_archive_sha256'] = record['candidate_archive_sha256']
    Path(destination).write_text(json.dumps(record, indent=2) + '\n', encoding='utf-8')
    return record
