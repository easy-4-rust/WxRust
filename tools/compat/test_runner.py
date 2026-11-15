"""Tests of evidence provenance and infrastructure-failure handling, not SDK parity."""
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

import run_diff


class ProvenanceTests(unittest.TestCase):
    def setUp(self):
        self.assertTrue(callable(getattr(run_diff, 'verify_java_provenance', None)),
                        'Actual Java class loading locations must be verified')

    def test_only_pinned_class_directories_are_accepted(self):
        with tempfile.TemporaryDirectory() as tmp:
            expected = [Path(tmp)/'common classes', Path(tmp)/'pay classes']
            log = '\n'.join([f'ORACLE_CLASS_SOURCE={expected[0].as_uri()}/',
                             f'PAY_CLASS_SOURCE={expected[1].as_uri()}/'])
            run_diff.verify_java_provenance(log, expected)
            with self.assertRaises(ValueError):
                run_diff.verify_java_provenance(log.replace(expected[0].as_uri(), (Path(tmp)/'other-version.jar').as_uri()), expected)
            with self.assertRaises(ValueError):
                run_diff.verify_java_provenance('ORACLE_CLASS_SOURCE='+expected[0].as_uri(), expected)
            with self.assertRaises(ValueError):
                run_diff.verify_java_provenance(log+'\n'+log, expected)


class RunnerFailureTests(unittest.TestCase):
    def test_missing_oracle_writes_infrastructure_error(self):
        import json
        with tempfile.TemporaryDirectory() as tmp:
            out = Path(tmp)/'evidence'
            process = subprocess.run([sys.executable, str(Path(run_diff.__file__)),
                '--java-root', str(Path(tmp)/'missing-oracle'), '--out', str(out)],
                capture_output=True, text=True, timeout=30)
            self.assertEqual(process.returncode, 2)
            report = json.loads((out/'report.json').read_text(encoding='utf-8'))
            self.assertFalse(report['passed'])
            self.assertEqual(report['status'], 'INFRA_ERROR')

    def test_existing_evidence_is_not_overwritten(self):
        with tempfile.TemporaryDirectory() as tmp:
            sentinel = Path(tmp)/'report.json'
            sentinel.write_text('original evidence', encoding='utf-8')
            process = subprocess.run([sys.executable, str(Path(run_diff.__file__)),
                '--java-root', tmp, '--out', tmp], capture_output=True, text=True, timeout=30)
            self.assertEqual(process.returncode, 2)
            self.assertEqual(sentinel.read_text(encoding='utf-8'), 'original evidence')

    def test_child_failure_is_not_a_comparison_pass(self):
        with tempfile.TemporaryDirectory() as tmp:
            with self.assertRaises(RuntimeError):
                run_diff.run_step([sys.executable, '-c', 'import sys;sys.exit(7)'],
                                  Path(tmp), Path(tmp), 'intentional-failure', dict(os.environ))
            self.assertTrue((Path(tmp)/'intentional-failure.stderr.log').is_file())


if __name__ == '__main__':
    unittest.main()
