"""Packaged graph rejection tests without uploads or dependency builds."""
import io
import json
from pathlib import Path
import tarfile
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

import registry_package_audit as audit


class PackagedRegistryTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix='sapid-packaged-audit-')
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.archive = self.root / 'sample.crate'
        self.output = self.root / 'evidence'
        self.manifest = b'[package]\nname="sample"\nversion="1.0.0"\n'
        self.lock = b'version = 4\n[[package]]\nname="sample"\nversion="1.0.0"\n'
        self.write_archive()
        self.report = {'vulnerabilities': {'found': False, 'count': 0, 'list': []},
                       'settings': {}, 'database': {'last-commit': None}}

    def write_archive(self, *, omit=(), commit='a' * 40):
        files = {'Cargo.toml': self.manifest, 'Cargo.lock': self.lock,
                 '.cargo_vcs_info.json': json.dumps({'git': {'sha1': commit}}).encode()}
        with tarfile.open(self.archive, 'w:gz') as archive:
            for name, content in files.items():
                if name not in omit:
                    entry = tarfile.TarInfo('sample-1.0.0/' + name)
                    entry.size = len(content)
                    archive.addfile(entry, io.BytesIO(content))

    def produce(self):
        def capture(command, **kwargs):
            return 'cargo-audit 0.22.2\n' if command[0] == 'auditor' else 'd' * 40
        response = SimpleNamespace(returncode=0, stdout=json.dumps(self.report))
        with patch.object(audit.subprocess, 'check_output', side_effect=capture), \
             patch.object(audit.subprocess, 'run', return_value=response):
            return audit.audit_archive(self.archive, self.output, 'auditor', self.root / 'db', 'a' * 40)

    def test_audits_packaged_source_without_resolving_an_unpublished_version(self):
        proof = self.produce()
        self.assertEqual(proof['kind'], 'packaged-source-registry-graph')
        self.assertEqual(proof['lock_sha256'], audit.sha(self.lock))
        audit.verify(self.output, 'a' * 40, 'd' * 40, self.archive)

    def test_missing_package_lock_wrong_source_and_nonregistry_graph_refuse_before_tool(self):
        with patch.object(audit.subprocess, 'run') as tool, \
             patch.object(audit.subprocess, 'check_output') as commands:
            self.write_archive(omit=('Cargo.lock',))
            with self.assertRaises(KeyError):
                audit.audit_archive(self.archive, self.output, 'auditor', self.root / 'db', 'a' * 40)
            commands.assert_not_called()
            self.write_archive(commit='b' * 40)
            with self.assertRaisesRegex(ValueError, 'frozen source'):
                audit.package_inputs(self.archive, 'a' * 40)
            self.lock += b'[[package]]\nname="path-glory"\nversion="0.7.0"\n'
            self.write_archive()
            with self.assertRaisesRegex(ValueError, 'crates.io-only'):
                audit.package_inputs(self.archive, 'a' * 40)
            tool.assert_not_called()

    def test_non_normalized_manifest_is_rejected(self):
        self.manifest += b'[dependencies.glory]\nversion="=0.7.0"\npath="../glory"\n'
        self.write_archive()
        with self.assertRaisesRegex(ValueError, 'path or Git'):
            audit.package_inputs(self.archive, 'a' * 40)

    def test_vulnerable_or_filtered_report_cannot_receive_successful_proof(self):
        for field, value in [('vulnerabilities', {'found': True, 'count': 1, 'list': ['vulnerable']}),
                             ('settings', {'ignore': ['RUSTSEC-test']})]:
            self.produce()  # A failed rescan must retire a previous success proof.
            original = self.report[field]
            self.report[field] = value
            with self.assertRaisesRegex(ValueError, 'failed or filtered'):
                self.produce()
            self.assertFalse((self.output / 'registry-package.json').exists())
            self.assertTrue((self.output / 'registry-audit.json').exists())
            self.report[field] = original

    def test_repackaged_manifest_and_lock_drift_stop_before_upload(self):
        self.produce()
        original = self.manifest
        self.manifest += b'[features]\nextra=[]\n'
        self.write_archive()
        with self.assertRaisesRegex(ValueError, 'Repackaged'):
            audit.verify(self.output, 'a' * 40, 'd' * 40, self.archive)
        self.manifest = original
        self.lock += b'# independently resolved package lock\n'
        self.write_archive()
        with self.assertRaisesRegex(ValueError, 'Repackaged'):
            audit.verify(self.output, 'a' * 40, 'd' * 40, self.archive)

    def test_wrong_database_and_tampered_raw_output_are_rejected(self):
        self.produce()
        with self.assertRaisesRegex(ValueError, 'advisory database'):
            audit.verify(self.output, 'a' * 40, 'e' * 40)
        (self.output / 'registry-audit.json').write_text('{}')
        with self.assertRaisesRegex(ValueError, 'digest differs'):
            audit.verify(self.output, 'a' * 40, 'd' * 40)


if __name__ == '__main__':
    unittest.main()
