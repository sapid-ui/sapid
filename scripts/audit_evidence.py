"""Bind successful RustSec scans to the product, dependency sources and used locks."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import registry_package_audit as registry

LOCKS = ('Cargo.lock', 'examples/web/Cargo.lock', 'examples/native/Cargo.lock')


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def load(path):
    return json.loads(Path(path).read_text(encoding='utf-8-sig'))


def commit(root):
    value = subprocess.check_output(['git', '-C', str(root), 'rev-parse', 'HEAD'], text=True).strip()
    if not re.fullmatch(r'[a-f0-9]{40}', value):
        raise ValueError('Invalid source commit')
    return value


def scans(directory):
    directory = Path(directory)
    report = load(directory / 'results.json')
    if (report.get('complete') is not True or report.get('errors') != []
            or report.get('tool') != 'cargo-audit 0.22.2' or len(report.get('locks', [])) != 3):
        raise ValueError('Dependency audit is incomplete or failed')
    databases = set()
    for index, (entry, name) in enumerate(zip(report['locks'], LOCKS)):
        if (entry.get('lockfile', '').replace('\\', '/') != name or entry.get('status') != 'passed'
                or entry.get('vulnerabilities') != [] or entry.get('snapshot') != f'audit-{index}.lock'
                or digest(directory / entry['snapshot']) != str(entry.get('sha256', '')).lower()
                or not re.fullmatch(r'[a-f0-9]{40}', entry.get('database', ''))):
            raise ValueError('Dependency audit lock snapshot or result differs')
        databases.add(entry['database'])
    if len(databases) != 1:
        raise ValueError('Dependency audit database changed between scans')
    return report


def record(root, directory, subject=None, require_package=False):
    root, directory = Path(root).resolve(), Path(directory).resolve()
    report = scans(directory)
    pin = load(root / 'deploy/dependencies.json')['glory']
    sources = {name: commit(root.parent / name) for name in ('glory', 'apace', 'takia', 'wingy')}
    if sources['glory'] != pin['revision']:
        raise ValueError('Dependency audit Glory source differs from the product pin')
    for entry, name in zip(report['locks'], LOCKS):
        if digest(root / name) != entry['sha256'].lower():
            raise ValueError('Dependency audit differs from the actual used lockfile')
    evidence = {'schema': 1, 'product_commit': commit(root), 'sources': sources,
                'glory_repository': pin['repository'], 'results_sha256': digest(directory / 'results.json'),
                'workflow_run_id': os.environ.get('GITHUB_RUN_ID'),
                'workflow_run_attempt': os.environ.get('GITHUB_RUN_ATTEMPT')}
    if subject:
        evidence['subject_sha256'] = digest(subject)
    if require_package or (directory / 'registry-package.json').is_file():
        registry.verify(directory, evidence['product_commit'], report['locks'][0]['database'])
        evidence['registry_package_sha256'] = digest(directory / 'registry-package.json')
    (directory / 'provenance.json').write_text(json.dumps(evidence, indent=2) + '\n', encoding='utf-8')
    return evidence


def verify(root, directory, expected_commit, expected_digest=None, subject=None, current_locks=False, current_sources=False, require_package=False):
    root, directory = Path(root), Path(directory)
    evidence = load(directory / 'provenance.json')
    if expected_digest is not None and digest(directory / 'provenance.json') != expected_digest:
        raise ValueError('Dependency audit provenance digest differs from successful checks')
    report = scans(directory)
    pin = load(root / 'deploy/dependencies.json')['glory']
    if (evidence.get('schema') != 1 or evidence.get('product_commit') != expected_commit
            or evidence.get('sources', {}).get('glory') != pin['revision']
            or evidence.get('glory_repository') != pin['repository']
            or evidence.get('results_sha256') != digest(directory / 'results.json')
            or set(evidence.get('sources', {})) != {'glory', 'apace', 'takia', 'wingy'}
            or any(not re.fullmatch(r'[a-f0-9]{40}', value) for value in evidence['sources'].values())):
        raise ValueError('Dependency audit differs from the product, sources or scanned results')
    if os.environ.get('GITHUB_RUN_ID') and evidence.get('workflow_run_id') != os.environ['GITHUB_RUN_ID']:
        raise ValueError('Dependency audit belongs to another workflow run')
    if subject and evidence.get('subject_sha256') != digest(subject):
        raise ValueError('Dependency audit belongs to a different built artifact')
    if current_sources and any(commit(root.resolve().parent / name) != value
                               for name, value in evidence['sources'].items()):
        raise ValueError('Dependency source checkout differs from the audited sources')
    if current_locks:
        for entry, name in zip(report['locks'], LOCKS):
            if digest(root / name) != entry['sha256'].lower():
                raise ValueError('Dependency audit differs from the actual used lockfile')
    if require_package or evidence.get('registry_package_sha256'):
        if digest(directory / 'registry-package.json') != evidence.get('registry_package_sha256'):
            raise ValueError('Packaged registry audit digest differs from dependency provenance')
        registry.verify(directory, expected_commit, report['locks'][0]['database'])
    return evidence


def export(directory, destination):
    directory, destination = Path(directory), Path(destination)
    scans(directory)
    destination.mkdir(parents=True, exist_ok=True)
    names = ['results.json', 'provenance.json', *(f'audit-{index}.lock' for index in range(3))]
    if load(directory / 'provenance.json').get('registry_package_sha256'):
        names.extend(registry.FILES)
    for name in names:
        shutil.copyfile(directory / name, destination / name)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('mode', choices=['record', 'verify'])
    parser.add_argument('--root', type=Path, default=Path.cwd())
    parser.add_argument('--directory', type=Path, required=True)
    parser.add_argument('--expected-commit')
    parser.add_argument('--expected-digest')
    parser.add_argument('--subject', type=Path)
    parser.add_argument('--export', type=Path)
    parser.add_argument('--current-locks', action='store_true')
    parser.add_argument('--current-sources', action='store_true')
    parser.add_argument('--require-package', action='store_true')
    args = parser.parse_args()
    try:
        if args.mode == 'record':
            record(args.root, args.directory, args.subject, args.require_package)
        else:
            if not args.expected_commit:
                raise ValueError('Verification requires the frozen product commit')
            verify(args.root, args.directory, args.expected_commit, args.expected_digest,
                   args.subject, args.current_locks, args.current_sources, args.require_package)
        if args.export:
            export(args.directory, args.export)
        print(digest(args.directory / 'provenance.json'))
    except (OSError, ValueError, KeyError, TypeError) as error:
        raise SystemExit(f'Invalid dependency audit evidence: {error}') from error


if __name__ == '__main__':
    main()
