"""Audit Cargo's packaged registry graph before the crate is published."""
import hashlib
import argparse
import json
from pathlib import Path
from pathlib import PurePosixPath
import re
import subprocess
import tarfile
import tomllib

FILES = ('registry-package.json', 'registry-package.toml', 'registry-package.lock', 'registry-audit.json')
METADATA = {'Cargo.toml', 'Cargo.lock', 'Cargo.toml.orig', '.cargo_vcs_info.json'}


def sha(payload):
    return hashlib.sha256(payload).hexdigest()


def package_inputs(archive, expected_commit, *, normalized_publication=False):
    with tarfile.open(archive) as source:
        members = source.getmembers()
        seen = set()
        for entry in members:
            path = PurePosixPath(entry.name)
            if (entry.name in seen or not entry.isfile() or path.is_absolute()
                    or any(part in ('.', '..') for part in entry.name.split('/'))
                    or '\\' in entry.name or ':' in entry.name):
                raise ValueError('Package contains duplicate, unsafe or nonregular members')
            seen.add(entry.name)
        manifests = [entry for entry in members if entry.name.count('/') == 1
                     and entry.name.endswith('/Cargo.toml')]
        if len(manifests) != 1:
            raise ValueError('Package requires one normalized manifest')
        prefix = manifests[0].name.rsplit('/', 1)[0]
        if any(not entry.name.startswith(prefix + '/') for entry in members):
            raise ValueError('Package member lies outside its source prefix')
        sources = {entry.name.removeprefix(prefix + '/'): sha(source.extractfile(entry).read())
                   for entry in members if entry.name.removeprefix(prefix + '/') not in METADATA}
        contents = {}
        required = ['Cargo.toml', 'Cargo.lock', '.cargo_vcs_info.json', 'Cargo.toml.orig']
        for name in required:
            entry = source.getmember(f'{prefix}/{name}')
            if not entry.isfile() or entry.size > 16 * 1024 * 1024:
                raise ValueError('Invalid packaged registry input')
            contents[name] = source.extractfile(entry).read()
    manifest = tomllib.loads(contents['Cargo.toml'].decode('utf-8'))
    package = manifest['package']
    if prefix != f"{package['name']}-{package['version']}":
        raise ValueError('Package prefix differs from normalized manifest')
    vcs_info = json.loads(contents['.cargo_vcs_info.json'])
    vcs = vcs_info['git']
    if 'dirty' in vcs and not isinstance(vcs['dirty'], bool):
        raise ValueError('Invalid packaged VCS dirty state')
    if normalized_publication and contents['Cargo.toml.orig'] != contents['Cargo.toml']:
        raise ValueError('Normalized publication original manifest differs from the audited input')
    if vcs.get('sha1') != expected_commit or (vcs.get('dirty') and not normalized_publication):
        raise ValueError('Packaged registry graph differs from the frozen source commit')
    sections = [manifest, *manifest.get('target', {}).values()]
    for section in sections:
        for kind in ('dependencies', 'dev-dependencies', 'build-dependencies'):
            if any(isinstance(dep, dict) and any(key in dep for key in ('path', 'git'))
                   for dep in section.get(kind, {}).values()):
                raise ValueError('Packaged manifest retains a path or Git dependency')
    lock = tomllib.loads(contents['Cargo.lock'].decode('utf-8'))
    roots = []
    for entry in lock['package']:
        if entry.get('source') == 'registry+https://github.com/rust-lang/crates.io-index':
            continue
        if (entry['name'], entry['version'], entry.get('source')) != (package['name'], package['version'], None):
            raise ValueError('Packaged dependency graph is not crates.io-only')
        roots.append(entry)
    if len(roots) != 1:
        raise ValueError('Packaged lock requires exactly one source package')
    return contents['Cargo.toml'], contents['Cargo.lock'], sources, vcs_info, sha(contents['Cargo.toml.orig'])


def clean_report(report):
    settings = report['settings']
    if (report['vulnerabilities']['found'] or report['vulnerabilities']['count'] != 0
            or report['vulnerabilities']['list'] or settings.get('ignore')
            or settings.get('target_arch') or settings.get('target_os')
            or settings.get('severity') is not None):
        raise ValueError('Packaged registry dependency audit failed or filtered advisories')


def audit_archive(archive, directory, auditor, database, expected_commit):
    directory, database = Path(directory), Path(database)
    directory.mkdir(parents=True, exist_ok=True)
    (directory / 'registry-package.json').unlink(missing_ok=True)
    manifest, lock, sources, vcs_info, original_manifest = package_inputs(archive, expected_commit)
    version = subprocess.check_output([str(auditor), '--version'], text=True).strip()
    if version != 'cargo-audit 0.22.2':
        raise ValueError('Packaged registry audit requires cargo-audit 0.22.2')
    directory.mkdir(parents=True, exist_ok=True)
    (directory / 'registry-package.toml').write_bytes(manifest)
    (directory / 'registry-package.lock').write_bytes(lock)
    result = subprocess.run([str(auditor), 'audit', '--file', str(directory / 'registry-package.lock'),
                             '--db', str(database), '--no-fetch', '--json'],
                            capture_output=True, text=True, encoding='utf-8')
    raw = result.stdout.encode('utf-8')
    (directory / 'registry-audit.json').write_bytes(raw)
    report = json.loads(raw)
    clean_report(report)
    if result.returncode:
        raise ValueError('Packaged registry dependency audit exited unsuccessfully')
    revision = subprocess.check_output(['git', '-C', str(database), 'rev-parse', 'HEAD'], text=True).strip()
    if not re.fullmatch(r'[a-f0-9]{40}', revision):
        raise ValueError('Packaged registry audit database is not immutable')
    if report['database'].get('last-commit') not in (None, revision):
        raise ValueError('Packaged registry audit differs from the database checkout')
    proof = {'schema': 1, 'kind': 'packaged-source-registry-graph', 'source_commit': expected_commit,
             'tool': version, 'database': revision, 'manifest_sha256': sha(manifest),
             'lock_sha256': sha(lock), 'report_sha256': sha(raw), 'source_files_sha256': sources,
             'publication_transform': 'normalized-manifest-and-registry-lock', 'vcs_info': vcs_info,
             'original_manifest_sha256': original_manifest}
    (directory / 'registry-package.json').write_text(json.dumps(proof, indent=2) + '\n', encoding='utf-8')
    return proof


def verify(directory, expected_commit, database, archive=None, *, normalized_publication=False):
    directory = Path(directory)
    proof = json.loads((directory / 'registry-package.json').read_text(encoding='utf-8'))
    if (proof.get('schema') != 1 or proof.get('kind') != 'packaged-source-registry-graph'
            or proof.get('source_commit') != expected_commit or proof.get('database') != database
            or proof.get('tool') != 'cargo-audit 0.22.2'
            or proof.get('publication_transform') != 'normalized-manifest-and-registry-lock'
            or not isinstance(proof.get('source_files_sha256'), dict)
            or not isinstance(proof.get('vcs_info'), dict)):
        raise ValueError('Packaged registry audit differs from source or advisory database')
    for key, name in [('manifest_sha256', 'registry-package.toml'), ('lock_sha256', 'registry-package.lock'),
                      ('report_sha256', 'registry-audit.json')]:
        if sha((directory / name).read_bytes()) != proof.get(key):
            raise ValueError('Packaged registry audit input or report digest differs')
    clean_report(json.loads((directory / 'registry-audit.json').read_bytes()))
    if archive:
        manifest, lock, sources, vcs_info, original_manifest = package_inputs(archive, expected_commit,
                                               normalized_publication=normalized_publication)
        if sha(manifest) != proof['manifest_sha256'] or sha(lock) != proof['lock_sha256']:
            raise ValueError('Repackaged registry dependency graph differs from the audited package')
        if not normalized_publication and original_manifest != proof.get('original_manifest_sha256'):
            raise ValueError('Original package manifest differs from the audited source archive')
        expected_vcs = json.loads(json.dumps(proof['vcs_info']))
        actual_vcs = json.loads(json.dumps(vcs_info))
        expected_vcs['git'].pop('dirty', None)
        actual_vcs['git'].pop('dirty', None)
        if actual_vcs != expected_vcs:
            raise ValueError('Publication VCS metadata differs beyond the allowed dirty state')
        if sources != proof['source_files_sha256']:
            expected_sources = proof['source_files_sha256']
            added = sorted(sources.keys() - expected_sources.keys())
            missing = sorted(expected_sources.keys() - sources.keys())
            changed = sorted(name for name in sources.keys() & expected_sources.keys()
                             if sources[name] != expected_sources[name])
            raise ValueError(f'Publication source files differ from the audited package: '
                             f'added={added}; missing={missing}; changed={changed}')
    return proof


def verify_published(directory, expected_commit, database, archive):
    """Accept an original clean release or the explicitly constrained transform."""
    with tarfile.open(archive) as packaged:
        entries = [entry for entry in packaged.getmembers() if entry.name.endswith('/.cargo_vcs_info.json')]
        if len(entries) != 1:
            raise ValueError('Published package requires unique VCS metadata')
        dirty = json.load(packaged.extractfile(entries[0]))['git'].get('dirty', False)
    verify(directory, expected_commit, database, archive, normalized_publication=dirty is True)
    return dirty is True


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--archive', type=Path, required=True)
    parser.add_argument('--directory', type=Path, required=True)
    parser.add_argument('--auditor', required=True)
    parser.add_argument('--database', type=Path, required=True)
    parser.add_argument('--expected-commit', required=True)
    args = parser.parse_args()
    try:
        audit_archive(args.archive, args.directory, args.auditor, args.database, args.expected_commit)
    except (OSError, ValueError, KeyError, TypeError, tarfile.TarError) as error:
        raise SystemExit(f'Invalid packaged registry audit: {error}') from error


if __name__ == '__main__':
    main()
