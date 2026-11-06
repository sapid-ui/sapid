"""Conservative CI scope from the complete Git diff; unknown paths require all checks."""
import argparse
import json
import os
from pathlib import PurePosixPath
import re
import subprocess

DOC_ROOTS = {"README.md", "AGENTS.md", "RELEASING.md", "CONTRIBUTING.md", "CHANGELOG.md", "SECURITY.md", "CODE_OF_CONDUCT.md"}
SITE_FILES = {"examples/web/site.css", "examples/web/showcase.html", "examples/web/showcase-acceptance.js"}
SITE_RUST_FILES = {"examples/web/src/showcase.rs", "examples/web/src/bin/showcase_ssr.rs", "examples/web/src/bin/theme_audit.rs", "examples/web/tests/showcase_site.rs", "examples/web/tests/showcase_routing.rs"}
MEDIA_SUFFIXES = {".svg", ".png", ".jpg", ".jpeg", ".gif", ".webp", ".avif", ".ico", ".woff", ".woff2", ".ttf", ".otf", ".mp4", ".webm", ".mp3", ".ogg"}
SHA = re.compile(r"[0-9a-f]{40}")


def is_doc(path):
    return path in DOC_ROOTS or (path.endswith(".md") and path.startswith(("docs/", "releases/"))) or path == "examples/web/README.md"


def is_site(path):
    return (path in SITE_FILES or path in SITE_RUST_FILES
        or (path.endswith(".rs") and path.startswith(("examples/web/src/showcase_", "examples/web/src/focused_demos/", "examples/web/src/dedicated_demos/")))
        or (path.startswith("examples/web/assets/") and PurePosixPath(path).suffix in MEDIA_SUFFIXES)
        or (path.startswith("scripts/showcase-remediation-") and path.endswith(".cjs")))


def classify(paths, force=False):
    paths = sorted(set(paths))
    unsafe = any(not p or "\\" in p or p.startswith("/") or any(part in (".", "..") for part in p.split("/")) for p in paths)
    full = force or unsafe or any(not is_doc(p) and not is_site(p) for p in paths)
    site = full or any(is_site(p) for p in paths)
    return {"rust": full, "showcase": site,
            "showcase_rust": full or any(is_site(p) and p.endswith(".rs") for p in paths),
            "docs": full or any(is_doc(p) for p in paths), "paths": paths}


def git(*args):
    return subprocess.check_output(["git", *args], stderr=subprocess.DEVNULL)


def plan(base, head, event, release_ref="", ref=""):
    force = bool(release_ref) or ref.startswith("refs/tags/") or event not in ("push", "pull_request")
    resolved_base = ""
    paths = []
    try:
        if not SHA.fullmatch(base) or set(base) == {"0"} or not SHA.fullmatch(head):
            raise ValueError("missing immutable diff endpoints")
        git("cat-file", "-e", base + "^{commit}")
        git("cat-file", "-e", head + "^{commit}")
        resolved_base = git("merge-base", base, head).decode().strip() if event == "pull_request" else base
        # No rename detection: both deleted and added paths must be classified.
        paths = [p.decode("utf-8") for p in git("diff", "--no-ext-diff", "--no-renames", "--name-only", "-z", resolved_base, head).split(b"\0") if p]
    except (subprocess.CalledProcessError, ValueError, UnicodeError):
        force = True
    result = classify(paths, force)
    result["base"] = resolved_base
    result["reason"] = "full/release-or-unknown" if result["rust"] else "showcase" if result["showcase"] else "documentation" if result["docs"] else "no changes"
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base", default=os.environ.get("BASE_SHA", ""))
    parser.add_argument("--head", default=os.environ.get("GITHUB_SHA", ""))
    parser.add_argument("--event", default=os.environ.get("GITHUB_EVENT_NAME", ""))
    parser.add_argument("--release-ref", default=os.environ.get("RELEASE_REF", ""))
    args = parser.parse_args()
    result = plan(args.base, args.head, args.event, args.release_ref, os.environ.get("GITHUB_REF", ""))
    print(json.dumps(result, indent=2))
    if output := os.environ.get("GITHUB_OUTPUT"):
        with open(output, "a", encoding="utf-8") as file:
            for key in ("rust", "showcase", "showcase_rust", "docs", "base"):
                file.write(f"{key}={str(result[key]).lower()}\n")
    if summary := os.environ.get("GITHUB_STEP_SUMMARY"):
        with open(summary, "a", encoding="utf-8") as file:
            file.write("### CI scope: " + result["reason"] + "\n\n```json\n" + json.dumps(result, indent=2) + "\n```\n")


if __name__ == "__main__":
    main()
