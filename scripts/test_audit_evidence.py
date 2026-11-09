"""Audit provenance rejection tests without network, builds or registry writes."""
import copy
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import audit_evidence as audit


class AuditEvidenceTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix='sapid-audit-evidence-')
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name) / 'product'
        self.directory = Path(temporary.name) / 'audit'
        self.root.mkdir()
        self.directory.mkdir()
        (self.root / 'deploy').mkdir()
        (self.root / 'deploy/dependencies.json').write_text(json.dumps({
            'glory': {'repository': 'example/glory', 'revision': 'b' * 40}}))
        self.subject = self.root / 'asset-hashes.json'
        self.subject.write_text('actual built asset manifest')
        self.report = {'tool': 'cargo-audit 0.22.2', 'complete': True, 'errors': [], 'locks': []}
        for index, name in enumerate(audit.LOCKS):
            lock = self.root / name
            lock.parent.mkdir(parents=True, exist_ok=True)
            lock.write_text(f'lock {index}')
            snapshot = self.directory / f'audit-{index}.lock'
            snapshot.write_bytes(lock.read_bytes())
            self.report['locks'].append({'lockfile': name, 'status': 'passed',
                'vulnerabilities': [], 'database': 'd' * 40,
                'snapshot': snapshot.name, 'sha256': audit.digest(snapshot)})
        self.save_report()
        self.commits = patch.object(audit, 'commit', side_effect=lambda root: 'a' * 40 if root == self.root else 'b' * 40)
        self.commits.start()
        self.addCleanup(self.commits.stop)
        self.environment = patch.dict(os.environ, {'GITHUB_RUN_ID': '123', 'GITHUB_RUN_ATTEMPT': '2'})
        self.environment.start()
        self.addCleanup(self.environment.stop)
        audit.record(self.root, self.directory, self.subject)

    def save_report(self):
        (self.directory / 'results.json').write_text(json.dumps(self.report))

    def verify(self, **kwargs):
        return audit.verify(self.root, self.directory, 'a' * 40, **kwargs)

    def test_exact_successful_product_and_built_asset_accept(self):
        self.verify(expected_digest=audit.digest(self.directory / 'provenance.json'),
                    subject=self.subject, current_locks=True)

    def test_upstream_main_cannot_be_recorded_as_the_product_pin(self):
        with patch.object(audit, 'commit', return_value='c' * 40):
            with self.assertRaisesRegex(ValueError, 'source differs'):
                audit.record(self.root, self.directory)

    def test_failed_missing_and_vulnerable_scans_never_receive_provenance(self):
        original = copy.deepcopy(self.report)
        for mutate in [lambda value: value.update(complete=False),
                       lambda value: value['locks'].pop(),
                       lambda value: value['locks'][0].update(vulnerabilities=[{'id': 'RUSTSEC-test'}])]:
            self.report = copy.deepcopy(original)
            mutate(self.report)
            self.save_report()
            with self.assertRaises(ValueError):
                audit.record(self.root, self.directory)

    def test_changed_or_missing_lock_snapshot_rejected(self):
        snapshot = self.directory / 'audit-1.lock'
        snapshot.write_text('different resolution')
        with self.assertRaisesRegex(ValueError, 'snapshot'):
            self.verify()
        snapshot.unlink()
        with self.assertRaises(OSError):
            self.verify()

    def test_different_current_resolution_is_not_audited(self):
        (self.root / audit.LOCKS[1]).write_text('different resolver output')
        with self.assertRaisesRegex(ValueError, 'actual used lockfile'):
            self.verify(current_locks=True)

    def test_product_report_database_and_run_mismatches_rejected(self):
        path = self.directory / 'provenance.json'
        original = audit.load(path)
        for field, value in [('product_commit', 'c' * 40), ('results_sha256', 'f' * 64),
                             ('workflow_run_id', '456')]:
            changed = {**original, field: value}
            path.write_text(json.dumps(changed))
            with self.assertRaises(ValueError):
                self.verify()
        path.write_text(json.dumps(original))
        self.report['locks'][2]['database'] = 'e' * 40
        self.save_report()
        with self.assertRaisesRegex(ValueError, 'database changed'):
            self.verify()

    def test_publisher_source_drift_is_rejected(self):
        self.verify(current_sources=True)
        with patch.object(audit, 'commit', return_value='c' * 40):
            with self.assertRaisesRegex(ValueError, 'source checkout differs'):
                self.verify(current_sources=True)

    def test_untrusted_digest_and_different_built_artifact_rejected(self):
        with self.assertRaisesRegex(ValueError, 'digest differs'):
            self.verify(expected_digest='0' * 64)
        self.subject.write_text('other built artifact')
        with self.assertRaisesRegex(ValueError, 'different built artifact'):
            self.verify(subject=self.subject)

    def test_export_contains_only_finite_evidence_and_is_verifiable(self):
        destination = self.root / 'accepted/dependency-audit'
        audit.export(self.directory, destination)
        self.assertEqual(len(list(destination.iterdir())), 5)
        audit.verify(self.root, destination, 'a' * 40, subject=self.subject)


if __name__ == '__main__':
    unittest.main()
