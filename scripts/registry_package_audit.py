"""Audit Cargo's packaged registry graph before the crate is published."""
import hashlib
import argparse
import json
from pathlib import Path
import re
import subprocess
import tarfile
import tomllib

FILES = ('registry-package.json', 'registry-package.toml', 'registry-package.lock', 'registry-audit.json')


def sha(payload):
    return hashlib.sha256(payload).hexdigest()


def package_inputs(archive, expected_commit):
    with tarfile.open(archive) as source:
        manifests = [entry for entry in source.getmembers() if entry.name.count('/') == 1
                     and entry.name.endswith('/Cargo.toml')]
        if len(manifests) != 1:
            raise ValueError('Package requires one normalized manifest')
        prefix = manifests[0].name.rsplit('/', 1)[0]
        contents = {}
        for name in ('Cargo.toml', 'Cargo.lock', '.cargo_vcs_info.json'):
            entry = source.getmember(f'{prefix}/{name}')
            if not entry.isfile() or entry.size > 16 * 1024 * 1024:
                raise ValueError('Invalid packaged registry input')
            contents[name] = source.extractfile(entry).read()
    manifest = tomllib.loads(contents['Cargo.toml'].decode('utf-8'))
    package = manifest['package']
    if prefix != f"{package['name']}-{package['version']}":
        raise ValueError('Package prefix differs from normalized manifest')
    vcs = json.loads(contents['.cargo_vcs_info.json'])['git']
    if vcs.get('sha1') != expected_commit or vcs.get('dirty'):
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
    return contents['Cargo.toml'], contents['Cargo.lock']


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
    manifest, lock = package_inputs(archive, expected_commit)
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
             'lock_sha256': sha(lock), 'report_sha256': sha(raw)}
    (directory / 'registry-package.json').write_text(json.dumps(proof, indent=2) + '\n', encoding='utf-8')
    return proof


def verify(directory, expected_commit, database, archive=None):
    directory = Path(directory)
    proof = json.loads((directory / 'registry-package.json').read_text(encoding='utf-8'))
    if (proof.get('schema') != 1 or proof.get('kind') != 'packaged-source-registry-graph'
            or proof.get('source_commit') != expected_commit or proof.get('database') != database
            or proof.get('tool') != 'cargo-audit 0.22.2'):
        raise ValueError('Packaged registry audit differs from source or advisory database')
    for key, name in [('manifest_sha256', 'registry-package.toml'), ('lock_sha256', 'registry-package.lock'),
                      ('report_sha256', 'registry-audit.json')]:
        if sha((directory / name).read_bytes()) != proof.get(key):
            raise ValueError('Packaged registry audit input or report digest differs')
    clean_report(json.loads((directory / 'registry-audit.json').read_bytes()))
    if archive:
        manifest, lock = package_inputs(archive, expected_commit)
        if sha(manifest) != proof['manifest_sha256'] or sha(lock) != proof['lock_sha256']:
            raise ValueError('Repackaged registry dependency graph differs from the audited package')
    return proof


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
