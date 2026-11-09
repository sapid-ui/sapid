"""Real Cargo dry-run contrast for a path graph and the audited registry graph."""
import hashlib
import io
import json
import os
from pathlib import Path
import subprocess
import tarfile
import tempfile
import tomllib
import unittest
from unittest.mock import patch

import normalized_publication as publication
import registry_package_audit as audit


class NormalizedPublicationTests(unittest.TestCase):
    def test_registry_updates_cannot_change_the_isolated_publish_temporary_lock(self):
        with tempfile.TemporaryDirectory(prefix='sapid-normalized-publication-') as temporary, \
             patch.dict(os.environ, {'RUSTUP_TOOLCHAIN': '1.98.0'}):
            base = Path(temporary).resolve()
            root, registry = base / 'source', base / 'registry'
            root.mkdir()
            registry.mkdir()
            def registry_package(name, version, dependencies=()):
                manifest = f'[package]\nname="{name}"\nversion="{version}"\nedition="2021"\n'
                if dependencies:
                    manifest += '[dependencies]\n' + ''.join(f'{dep}="^1"\n' for dep in dependencies)
                archive = registry / f'{name}-{version}.crate'
                with tarfile.open(archive, 'w:gz') as packaged:
                    for file, payload in [('Cargo.toml', manifest.encode()), ('src/lib.rs', b'pub fn fixture() {}\n')]:
                        entry = tarfile.TarInfo(f'{name}-{version}/{file}')
                        entry.size = len(payload)
                        packaged.addfile(entry, io.BytesIO(payload))
                entry = {'name': name, 'vers': version, 'deps': [
                    {'name': dep, 'req': '^1', 'features': [], 'optional': False,
                     'default_features': True, 'target': None, 'kind': 'normal', 'registry': None}
                    for dep in dependencies], 'cksum': hashlib.sha256(archive.read_bytes()).hexdigest(),
                    'features': {}, 'yanked': False}
                index = registry / 'index' / name[:2] / name[2:4] / name
                index.parent.mkdir(parents=True, exist_ok=True)
                with index.open('a', encoding='utf-8') as output:
                    output.write(json.dumps(entry) + '\n')
            registry_package('publication-leaf', '1.0.0')
            registry_package('publication-child', '1.0.0', ['publication-leaf'])
            (root / 'src').mkdir()
            (root / 'src/lib.rs').write_text('pub fn source() {}\n', encoding='utf-8')
            (root / 'child/src').mkdir(parents=True)
            (root / 'child/src/lib.rs').write_text('pub fn child() {}\n', encoding='utf-8')
            (root / 'child/Cargo.toml').write_text(
                '[package]\nname="publication-child"\nversion="1.0.0"\nedition="2021"\n', encoding='utf-8')
            (root / 'Cargo.toml').write_text(
                '[package]\nname="publication-probe"\nversion="0.1.0"\nedition="2021"\n'
                '[workspace]\nresolver="3"\n[dependencies]\n'
                'publication-child={version="=1.0.0",path="child"}\n', encoding='utf-8')
            (root / '.gitignore').write_text('/target\n/Cargo.lock\n', encoding='utf-8')
            (root / '.gitattributes').write_text('* text=auto eol=lf\n', encoding='utf-8')
            (root / '.cargo').mkdir()
            (root / '.cargo/config.toml').write_text(
                '[source.crates-io]\nreplace-with="fixture"\n[source.fixture]\n'
                f'local-registry={json.dumps(registry.as_posix())}\n', encoding='utf-8')
            def run(args, directory=root):
                result = subprocess.run(args, cwd=directory, capture_output=True, text=True)
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                return result.stdout
            for args in [['git', 'init'], ['git', 'add', '.'], ['git', '-c', 'user.name=Fixture',
                         '-c', 'user.email=fixture@example.invalid', 'commit', '-m', 'Frozen source']]:
                run(args)
            commit = run(['git', 'rev-parse', 'HEAD']).strip()
            run(['cargo', 'generate-lockfile'])
            original_lock = (root / 'Cargo.lock').read_bytes()
            metadata = json.loads(run(['cargo', 'metadata', '--no-deps', '--format-version', '1']))
            target = Path(metadata['target_directory'])
            run(['cargo', 'package', '--locked', '--no-verify'])
            original = target / 'package/publication-probe-0.1.0.crate'
            evidence = base / 'audit'
            report = {'settings': {}, 'database': {}, 'vulnerabilities': {'found': False, 'count': 0, 'list': []}}
            with patch.object(audit.subprocess, 'check_output', side_effect=lambda args, **kwargs:
                    'cargo-audit 0.22.2' if args[0] == 'auditor' else 'd' * 40), \
                 patch.object(audit.subprocess, 'run', return_value=type('Result', (), {
                    'returncode': 0, 'stdout': json.dumps(report)})()):
                audit.audit_archive(original, evidence, 'auditor', base / 'db', commit)
            (evidence / 'provenance.json').write_text('{}', encoding='utf-8')
            registry_package('publication-leaf', '1.1.0')
            run(['cargo', 'publish', '--dry-run', '--no-verify', '--locked', '--registry', 'crates-io'])
            def leaf(archive):
                with tarfile.open(archive) as packaged:
                    lock = tomllib.loads(packaged.extractfile('publication-probe-0.1.0/Cargo.lock').read().decode())
                return next(entry['version'] for entry in lock['package'] if entry['name'] == 'publication-leaf')
            self.assertEqual(leaf(target / 'package/tmp-crate/publication-probe-0.1.0.crate'), '1.1.0')
            checkout, owned_target = publication.prepare(root, evidence, original, target, commit)
            self.assertEqual(run(['git', 'rev-parse', 'HEAD'], checkout).strip(), commit)
            with patch.object(publication.subprocess, 'run', wraps=subprocess.run) as calls:
                candidate = publication.candidate(checkout, owned_target, evidence, commit)
                commands = [call.args[0] for call in calls.call_args_list]
                self.assertTrue(all('--dry-run' in args for args in commands if args[:2] == ['cargo', 'publish']))
            self.assertEqual(leaf(candidate), '1.0.0')
            with self.assertRaisesRegex(ValueError, 'frozen source'):
                audit.verify(evidence, commit, 'd' * 40, candidate)
            record = publication.record_transform(evidence, candidate, commit, base / 'transform.json')
            self.assertTrue(record['vcs_dirty'])
            self.assertEqual(record['candidate_vcs_commit'], commit)
            (base / 'transform.json').unlink()
            resumed = publication.record_transform(evidence, candidate, commit, base / 'transform.json', published=True)
            self.assertEqual(resumed['artifact_stage'], 'downloaded-registry-archive')
            self.assertEqual(resumed['published_archive_sha256'], hashlib.sha256(candidate.read_bytes()).hexdigest())
            self.assertEqual((root / 'Cargo.lock').read_bytes(), original_lock)
            self.assertEqual(run(['git', 'status', '--porcelain']), '')


if __name__ == '__main__':
    unittest.main()
