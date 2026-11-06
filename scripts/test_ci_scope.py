"""Scope regressions use real Git histories; they never invoke Cargo or the network."""
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch
import ci_scope
import check_doc_links


class ScopeTests(unittest.TestCase):
    def test_documentation_never_selects_runtime_or_site_builds(self):
        result = ci_scope.classify(["README.md", "docs/color-picker.md", "releases/0.7.0.md"])
        self.assertEqual([result[k] for k in ("docs", "rust", "showcase", "showcase_rust")], [True, False, False, False])

    def test_shared_site_css_requires_browser_acceptance_without_rust_test_suites(self):
        result = ci_scope.classify(["examples/web/site.css", "scripts/showcase-remediation-color.cjs", "docs/color-picker.md"])
        self.assertEqual([result[k] for k in ("docs", "rust", "showcase", "showcase_rust")], [True, False, True, False])

    def test_showcase_rust_only_selects_related_web_rust_checks(self):
        for path in ["examples/web/src/showcase.rs", "examples/web/src/showcase_demos/rating_usage.rs", "examples/web/tests/showcase_site.rs"]:
            result = ci_scope.classify([path])
            self.assertFalse(result["rust"])
            self.assertTrue(result["showcase"] and result["showcase_rust"])

    def test_mixed_runtime_manifest_ci_and_unknown_paths_fail_closed(self):
        for path in ["src/color_input.rs", "styles/sapid.css", "Cargo.toml", "Cargo.lock", "deploy/dependencies.json", ".github/workflows/ci.yml", "scripts/ci_scope.py", "examples/web/src/lib.rs", "examples/fixtures/dsl_demo.rs", "unknown", "docs/../src/file.rs", "docs\\fake.md"]:
            with self.subTest(path=path):
                result = ci_scope.classify(["README.md", path])
                self.assertTrue(result["rust"] and result["showcase"] and result["showcase_rust"])
        self.assertTrue(ci_scope.classify(["examples/web/site.css"], force=True)["rust"])


class GitScopeTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="sapid-ci-scope-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.git("init", "-q")
        self.git("config", "core.autocrlf", "false")
        self.git("config", "user.email", "ci@example.invalid")
        self.git("config", "user.name", "CI fixture")
        self.write("README.md", "[Guide](docs/guide.md)\n")
        self.write("docs/guide.md", "# Guide\n")
        self.base = self.commit()
        original = Path.cwd()
        os.chdir(self.root)
        self.addCleanup(os.chdir, original)

    def git(self, *args):
        return subprocess.check_output(["git", *args], cwd=self.root).decode().strip()

    def write(self, name, text):
        file = self.root / name
        file.parent.mkdir(parents=True, exist_ok=True)
        file.write_text(text, encoding="utf-8")

    def commit(self):
        self.git("add", ".")
        self.git("commit", "-qm", "fixture")
        return self.git("rev-parse", "HEAD")

    def test_real_diff_includes_every_commit_and_both_rename_endpoints(self):
        self.write("examples/web/site.css", "body{}")
        self.commit()
        self.write("docs/notes.md", "notes")
        head = self.commit()
        self.assertFalse(ci_scope.plan(self.base, head, "push", validated_base=self.base)["rust"])
        self.assertTrue(ci_scope.plan(self.base, head, "pull_request", validated_base=self.base)["showcase"])
        self.git("mv", "docs/notes.md", "runtime.rs")
        self.assertTrue(ci_scope.plan(head, self.commit(), "push")["rust"])

    def test_css_after_an_unvalidated_runtime_push_cannot_skip_the_runtime_changes(self):
        self.write("src/lib.rs", "pub fn changed() {}")
        runtime = self.commit()
        self.write("examples/web/site.css", "body{}")
        head = self.commit()
        self.assertTrue(ci_scope.plan(runtime, head, "push", validated_base=self.base)["rust"])
        self.assertTrue(ci_scope.plan(runtime, head, "push")["rust"])
        self.assertFalse(ci_scope.plan(runtime, head, "push", validated_base=runtime)["rust"])

    def test_successful_baseline_requires_matching_completed_workflow_and_ancestry(self):
        valid = {"head_sha": self.base, "status": "completed", "conclusion": "success", "event": "push", "head_branch": "main", "path": ".github/workflows/ci.yml"}
        invalid = dict(valid, path=".github/workflows/other.yml")
        actual_git = subprocess.check_output
        def api(*args, **kwargs):
            if args[0][0] == "gh":
                import json
                return json.dumps({"workflow_runs": [invalid, valid]}).encode()
            return actual_git(*args, **kwargs)
        with patch.object(ci_scope.subprocess, "check_output", side_effect=api):
            self.assertEqual(ci_scope.successful_base(self.base, "main", "sapid-ui/sapid"), self.base)
        with patch.object(ci_scope.subprocess, "check_output", side_effect=OSError("offline")):
            self.assertEqual(ci_scope.successful_base(self.base, "main", "sapid-ui/sapid"), "")

    def test_missing_diffs_manual_schedules_and_release_refs_always_require_full(self):
        self.write("docs/notes.md", "notes")
        head = self.commit()
        for event, base, release, ref in [("push", "0"*40, "", ""), ("push", "f"*40, "", ""),
                ("schedule", self.base, "", ""), ("workflow_dispatch", self.base, "", ""),
                ("push", self.base, "a"*40, ""), ("push", self.base, "", "refs/tags/v0.7.0")]:
            self.assertTrue(ci_scope.plan(base, head, event, release, ref)["rust"])

    def test_new_bad_links_and_deleted_targets_fail_but_historical_broken_links_do_not(self):
        self.write("docs/history.md", "[Historical](removed.rs)\n")
        old = self.commit()
        self.assertEqual(check_doc_links.check(self.root, old), [])
        (self.root / "docs/guide.md").unlink()
        self.write("docs/new.md", "[Missing](missing.md)\n```md\n[Example](not-a-link.md)\n```\n")
        self.commit()
        errors = check_doc_links.check(self.root, old)
        self.assertEqual(len(errors), 2, errors)
        self.assertTrue(any("README.md" in error for error in errors))

    def test_deleted_compile_time_markdown_input_cannot_pass_documentation_checks(self):
        self.write("src/lib.rs", '#![doc = include_str!("../README.md")]\n')
        old = self.commit()
        (self.root / "README.md").unlink()
        self.commit()
        errors = check_doc_links.check(self.root, old)
        self.assertTrue(any("missing included documentation README.md" in error for error in errors), errors)

    def test_reference_html_encoded_paths_and_inline_examples(self):
        self.write("docs/has space.md", "# Present")
        self.write("docs/links.md", '[file](has%20space.md#heading)\n[file]: <has space.md>\n<img src="has%20space.md">\n`[sample](no.md)`\n')
        self.commit()
        self.assertEqual(check_doc_links.check(self.root, self.base), [])


if __name__ == "__main__":
    unittest.main()
