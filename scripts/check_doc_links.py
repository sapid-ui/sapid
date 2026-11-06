"""Reject new broken repository links and links broken by deleted documentation."""
import argparse
from functools import lru_cache
import html
import os
from pathlib import Path
import posixpath
import re
import subprocess
from urllib.parse import unquote, urlsplit


def links(text):
    fence = None
    for line in text.splitlines():
        marker = re.match(r"^ {0,3}(`{3,}|~{3,})", line)
        if marker:
            kind = marker[1][0]
            if fence is None:
                fence = (kind, len(marker[1]))
            elif fence[0] == kind and len(marker[1]) >= fence[1]:
                fence = None
            continue
        if fence:
            continue
        line = re.sub(r"`+[^`]*`+", "", line)
        for match in re.finditer(r"\[[^\]\n]*\]\(\s*(<[^>]+>|[^\s)]+)", line):
            yield html.unescape(match[1].strip("<>"))
        reference = re.match(r"^ {0,3}\[[^\]]+\]:\s*(<[^>]+>|\S+)", line)
        if reference:
            yield html.unescape(reference[1].strip("<>"))
        for match in re.finditer(r"(?:href|src)=[\"']([^\"']+)[\"']", line):
            yield html.unescape(match[1])


def repository_target(document, target):
    url = urlsplit(target)
    if url.scheme or url.netloc or not url.path:
        return None
    name = unquote(url.path)
    path = posixpath.normpath(name.lstrip("/") if name.startswith("/") else posixpath.join(posixpath.dirname(document), name))
    if path == ".." or path.startswith("../") or "\\" in path:
        return None  # Peer repositories and historical machine-local evidence.
    return path


def check(root, base):
    root = Path(root).resolve()
    def git(*args):
        return subprocess.run(["git", *args], cwd=root, check=True, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL).stdout
    if not re.fullmatch(r"[0-9a-f]{40}", base):
        try:
            base = git("rev-parse", "HEAD^").decode().strip()
        except subprocess.CalledProcessError:
            base = git("rev-parse", "HEAD").decode().strip()
    @lru_cache(maxsize=None)
    def existed(name):
        try:
            git("cat-file", "-e", base + ":" + name)
            return True
        except subprocess.CalledProcessError:
            return False
    failures = []
    # Markdown can be a compile-time input (notably the crate's README).
    # Check literal includes without compiling the Rust/native graph.
    for name in filter(None, git("ls-files", "-z", "--", "*.rs").decode().split("\0")):
        source = root / name
        if not source.is_file():
            continue
        for include in re.findall(r'include_(?:str|bytes)!\(\s*"([^"\n]+\.md)"\s*\)', source.read_text(encoding="utf-8")):
            target = repository_target(name, include)
            if target is not None and not (root / target).is_file():
                failures.append(f"{name}: missing included documentation {target}")
    names = git("ls-files", "-z", "--", "*.md").decode().split("\0")
    for name in filter(None, names):
        file = root / name
        if not file.is_file():
            continue
        try:
            old = git("show", base + ":" + name).decode("utf-8")
        except subprocess.CalledProcessError:
            old = ""
        old_targets = {repository_target(name, value) for value in links(old)}
        for target in {repository_target(name, value) for value in links(file.read_text(encoding="utf-8"))} - {None}:
            if not (root / target).exists() and (target not in old_targets or existed(target)):
                failures.append(f"{name}: missing {target}")
    return failures


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", default=".")
    parser.add_argument("--base", default="")
    args = parser.parse_args()
    failures = check(args.root, args.base)
    if failures:
        raise SystemExit("\n".join(sorted(failures)))
    print("Repository documentation links: no new broken targets")


if __name__ == "__main__":
    main()
