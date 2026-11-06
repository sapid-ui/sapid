"""Conservative CI scope from the complete Git diff; unknown paths require all checks."""
import argparse
import json
import os
from pathlib import PurePosixPath
import re
import subprocess
from urllib.parse import urlencode

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


def successful_base(head, branch, repository):
    """Use a successful ancestor, so a later CSS push cannot hide a failed Rust push."""
    if not re.fullmatch(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+", repository) or not SHA.fullmatch(head):
        return ""
    query = urlencode({"status": "success", "event": "push", "branch": branch, "per_page": 100})
    try:
        records = json.loads(subprocess.check_output(["gh", "api", f"repos/{repository}/actions/workflows/ci.yml/runs?{query}"], stderr=subprocess.DEVNULL))["workflow_runs"]
        for record in records:
            commit = record.get("head_sha", "")
            if (not SHA.fullmatch(commit) or record.get("status") != "completed"
                    or record.get("conclusion") != "success" or record.get("event") != "push"
                    or record.get("head_branch") != branch or record.get("path") != ".github/workflows/ci.yml"):
                continue
            if subprocess.run(["git", "merge-base", "--is-ancestor", commit, head], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).returncode == 0:
                return commit
    except (OSError, subprocess.CalledProcessError, ValueError, KeyError):
        pass
    return ""


def plan(base, head, event, release_ref="", ref="", validated_base=""):
    force = bool(release_ref) or ref.startswith("refs/tags/") or event not in ("push", "pull_request")
    force = force or not SHA.fullmatch(validated_base)
    resolved_base = ""
    paths = []
    try:
        if not SHA.fullmatch(base) or set(base) == {"0"} or not SHA.fullmatch(head):
            raise ValueError("missing immutable diff endpoints")
        git("cat-file", "-e", base + "^{commit}")
        git("cat-file", "-e", head + "^{commit}")
        resolved_base = git("merge-base", base, head).decode().strip() if event == "pull_request" else base
        if SHA.fullmatch(validated_base):
            # Include changes from unsuccessful/in-flight ancestor runs as well.
            resolved_base = git("merge-base", resolved_base, validated_base).decode().strip()
        # No rename detection: both deleted and added paths must be classified.
        paths = [p.decode("utf-8") for p in git("diff", "--no-ext-diff", "--no-renames", "--name-only", "-z", resolved_base, head).split(b"\0") if p]
    except (subprocess.CalledProcessError, ValueError, UnicodeError):
        force = True
    result = classify(paths, force)
    result["validated_base"] = validated_base
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
    ref = os.environ.get("GITHUB_REF", "")
    baseline = ""
    if not args.release_ref and not ref.startswith("refs/tags/") and args.event in ("push", "pull_request"):
        branch = os.environ.get("GITHUB_BASE_REF") or os.environ.get("GITHUB_REF_NAME", "")
        baseline = successful_base(args.head, branch, os.environ.get("GITHUB_REPOSITORY", ""))
    result = plan(args.base, args.head, args.event, args.release_ref, ref, baseline)
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
