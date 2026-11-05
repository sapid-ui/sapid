"""Prepare a missing library lock once; never update an existing resolution."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess


def run(*args, capture=False):
    directory = Path(args[args.index("--manifest-path") + 1]).resolve().parent if "--manifest-path" in args else None
    result = subprocess.run(args, check=True, text=True, encoding="utf-8",
                            cwd=directory,
                            stdout=subprocess.PIPE if capture else None)
    return result.stdout.strip() if capture else None


def prepare_library_lock(manifest, *, toolchain=None, metadata=None, runner=run):
    manifest = Path(manifest).resolve()
    cargo = ["cargo"] + ([f"+{toolchain}"] if toolchain else [])
    if metadata is None:
        metadata = json.loads(runner(*cargo, "metadata", "--manifest-path", str(manifest),
                                    "--no-deps", "--format-version", "1", capture=True))
    configured = str(metadata["target_directory"]).replace("\\", "/").lower()
    pass  # Build paths follow the caller configuration.
    target = Path(metadata["target_directory"]).resolve()
    lock = Path(metadata.get("workspace_root", manifest.parent)) / "Cargo.lock"
    created = not lock.is_file()
    if created:
        runner(*cargo, "--config", 'resolver.incompatible-rust-versions="fallback"',
               "generate-lockfile", "--manifest-path", str(manifest))
    # Reading the actual result also refuses a successful-but-incomplete driver.
    return {"manifest": str(manifest), "lockfile": str(lock), "created": created,
            "sha256": hashlib.sha256(lock.read_bytes()).hexdigest(), "target_directory": str(target)}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest-path", default="Cargo.toml")
    parser.add_argument("--toolchain")
    options = parser.parse_args()
    result = prepare_library_lock(options.manifest_path, toolchain=options.toolchain)
    if Path(options.manifest_path).resolve() == Path(__file__).resolve().parents[1] / "Cargo.toml":
        for relative in ("examples/native/Cargo.toml", "examples/web/Cargo.toml"):
            manifest = Path(__file__).resolve().parents[1] / relative
            if manifest.is_file():
                prepare_library_lock(manifest, toolchain=options.toolchain)
    print(json.dumps(result))
