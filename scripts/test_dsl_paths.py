"""Run the DSL gate's real path preparation and a real Windows Rust link smoke."""
from pathlib import Path as _BuildPath
import sys as _build_sys
_build_sys.path.insert(0, str(_BuildPath(__file__).resolve().parents[1] / "scripts"))
from local_build_paths import local_build_path
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[1]
PWSH = shutil.which("pwsh")
RUSTC = shutil.which("rustc")
PREPARE = r"""
$ErrorActionPreference = 'Stop'
. (Join-Path (Split-Path -Parent $env:DSL_TEST_SCRIPT) 'local-build-paths.ps1')
$tokens = $null; $errors = $null
$ast = [Management.Automation.Language.Parser]::ParseFile($env:DSL_TEST_SCRIPT, [ref]$tokens, [ref]$errors)
if ($errors.Count) { throw ($errors | Out-String) }
$function = $ast.Find({param($node) $node -is [Management.Automation.Language.FunctionDefinitionAst] -and $node.Name -eq 'Set-DslBuildPaths'}, $true)
if (-not $function) { throw 'Missing production path preparation function' }
Invoke-Expression $function.Extent.Text
$taskBuild = $env:DSL_TEST_BUILD_ROOT
if ($env:DSL_TEST_NATIVE -eq '1') {
    $branches = @($ast.FindAll({param($node) $node -is [Management.Automation.Language.IfStatementAst] -and $node.Clauses[0].Item1.Extent.Text -eq '$Native'}, $true))
    $branch = $branches[-1]
    $commands = @($branch.Clauses[0].Item2.FindAll({param($node) $node -is [Management.Automation.Language.CommandAst]}, $true))
    $prepare = $commands | Where-Object { $_.GetCommandName() -eq 'Set-DslBuildPaths' -and $_.Extent.Text -match '-Native' } | Select-Object -First 1
    $cargo = $commands | Where-Object { $_.GetCommandName() -in @('Confirm-Target','Invoke-Cargo','cargo') } | Select-Object -First 1
    if (-not $prepare -or -not $cargo -or $prepare.Extent.StartOffset -gt $cargo.Extent.StartOffset) {
        throw 'Native paths must be prepared before the first Cargo command'
    }
    # Execute the actual native-stage preparation command, not a fake Cargo driver.
    Invoke-Expression $prepare.Extent.Text
} else { Set-DslBuildPaths -Root $taskBuild }
@{ target = $env:CARGO_TARGET_DIR; temp = $env:TEMP; tmp = $env:TMP;
   site = $env:SAPID_DSL_SITE; cargo = $env:CARGO_HOME } | ConvertTo-Json -Compress
"""


@unittest.skipUnless(os.name == "nt" and PWSH and RUSTC, "Windows, PowerShell 7 and Rust are required")
class DslBuildPathTests(unittest.TestCase):
    def setUp(self):
        parent = Path(os.environ.get("BUILD_ROOT", local_build_path('sapid/fixes/gates'))) / "dsl-path-tests"
        parent.mkdir(parents=True, exist_ok=True)
        self.directory = tempfile.TemporaryDirectory(prefix="standalone-", dir=parent)
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name).resolve()
        self.build = self.root / "fresh-build"
        self.env = os.environ.copy()
        self.env.update(DSL_TEST_SCRIPT=str(ROOT / "scripts/check-dsl.ps1"),
                        DSL_TEST_BUILD_ROOT=str(self.build),
                        CARGO_HOME=os.environ.get("CARGO_HOME") or str(Path.home() / ".cargo"),
                        CARGO_TARGET_DIR=str(self.root / "target"),
                        TEMP=str(self.root / "tmp"), TMP=str(self.root / "tmp"))
        (self.root / "tmp").mkdir()

    def prepare(self, native=False, **overrides):
        env = self.env | {"DSL_TEST_NATIVE": "1" if native else "0"} | overrides
        result = subprocess.run([PWSH, "-NoProfile", "-Command", PREPARE], env=env,
                                capture_output=True, text=True, timeout=30)
        return result

    def test_fresh_native_stage_creates_temp_before_a_real_rust_link(self):
        source = self.root / "smoke.rs"
        source.write_text('fn main() { println!("DSL native link smoke"); }\n', encoding="utf-8")
        missing = self.root / "absent-temp"
        bad = subprocess.run([RUSTC, "--edition=2024", str(source), "-o", str(self.root / "bad.exe")],
                             env=self.env | {"TEMP": str(missing), "TMP": str(missing)},
                             capture_output=True, text=True, timeout=60)
        self.assertNotEqual(bad.returncode, 0, "The negative smoke must exercise a genuinely missing compiler TEMP")
        self.assertIn("temp", bad.stderr.lower(), bad.stderr)
        self.assertFalse(self.build.exists())
        prepared = self.prepare(native=True)
        self.assertEqual(prepared.returncode, 0, prepared.stdout + prepared.stderr)
        paths = json.loads(prepared.stdout)
        self.assertEqual(Path(paths["target"]), Path(self.env["CARGO_TARGET_DIR"]))
        self.assertEqual(Path(paths["temp"]), Path(self.env["TEMP"]))
        self.assertEqual(paths["temp"], paths["tmp"])
        self.assertEqual(paths["cargo"], self.env["CARGO_HOME"])
        self.assertTrue(Path(paths["target"]).is_dir())
        self.assertTrue(Path(paths["temp"]).is_dir())
        binary = Path(paths["target"]) / "smoke.exe"
        linked = subprocess.run([RUSTC, "--edition=2024", str(source), "-o", str(binary)],
                                env=self.env | {"TEMP": paths["temp"], "TMP": paths["tmp"]},
                                capture_output=True, text=True, timeout=60)
        self.assertEqual(linked.returncode, 0, linked.stdout + linked.stderr)
        executed = subprocess.run([str(binary)], capture_output=True, text=True, timeout=10)
        self.assertEqual(executed.returncode, 0, executed.stdout + executed.stderr)
        self.assertIn("DSL native link smoke", executed.stdout)

    def test_portable_and_native_paths_share_only_the_requested_build_root(self):
        portable = self.prepare()
        self.assertEqual(portable.returncode, 0, portable.stdout + portable.stderr)
        paths = json.loads(portable.stdout)
        self.assertEqual(Path(paths["target"]), Path(self.env["CARGO_TARGET_DIR"]))
        self.assertEqual(Path(paths["site"]), self.build / "dsl")
        self.assertTrue(Path(paths["temp"]).is_dir())
        native = self.prepare(native=True)
        self.assertEqual(native.returncode, 0, native.stdout + native.stderr)
        paths = json.loads(native.stdout)
        self.assertEqual(Path(paths["target"]), Path(self.env["CARGO_TARGET_DIR"]))
        self.assertEqual(Path(paths["temp"]), Path(self.env["TEMP"]))

    def test_explicit_output_directory_is_accepted(self):
        custom = self.root / "custom-output"
        result = self.prepare(native=True, DSL_TEST_BUILD_ROOT=str(custom))
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        paths = json.loads(result.stdout)
        self.assertEqual(Path(paths["site"]), custom / "dsl")
        self.assertEqual(Path(paths["target"]), Path(self.env["CARGO_TARGET_DIR"]))

    def test_caller_cargo_home_and_temp_are_preserved(self):
        custom = self.root / "caller-cargo-home"
        result = self.prepare(native=True, CARGO_HOME=str(custom))
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        paths = json.loads(result.stdout)
        self.assertEqual(Path(paths["cargo"]), custom)
        self.assertEqual(paths["temp"], self.env["TEMP"])
        self.assertEqual(paths["tmp"], self.env["TMP"])


if __name__ == "__main__":
    unittest.main()
