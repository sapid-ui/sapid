"""Fetch current official pages and prepare the one-to-one documentation review.

This is a read-only audit: it does not refresh the frozen API facts or alter UI.
Raw upstream documents and browser artifacts stay under project build directory/sapid.
"""
from pathlib import Path as _BuildPath
import sys as _build_sys
_build_sys.path.insert(0, str(_BuildPath(__file__).resolve().parents[1] / "scripts"))
from local_build_paths import local_build_path
import concurrent.futures
import datetime
import hashlib
from html.parser import HTMLParser
import json
import os
from pathlib import Path
import re
import time
from urllib.request import Request, urlopen

ROOT = Path(__file__).resolve().parents[1]
OUTPUT = Path(os.environ.get("SAPID_BROWSER_OUTPUT", local_build_path('sapid/shared-page-audit')))
OUTPUT.mkdir(parents=True, exist_ok=True)
(OUTPUT / "upstream").mkdir(exist_ok=True)


class Headings(HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.capture = None
        self.items = []

    def handle_starttag(self, tag, attrs):
        attributes = dict(attrs)
        if tag in {"h1", "h2", "h3"} and (
            "MdxTitle" in attributes.get("class", "")
            or "MdxPageHeader" in attributes.get("class", "")
        ):
            self.capture = {"level": int(tag[1]), "text": "", "anchor": None}
        if tag == "a" and self.capture and attributes.get("href", "").startswith("#"):
            self.capture["anchor"] = attributes["href"]

    def handle_data(self, data):
        if self.capture:
            self.capture["text"] += data

    def handle_endtag(self, tag):
        if self.capture and tag == f"h{self.capture['level']}":
            self.capture["text"] = " ".join(self.capture["text"].split())
            self.items.append(self.capture)
            self.capture = None


def fetch(entry):
    result = dict(entry)
    path = OUTPUT / "upstream" / f"{entry['slug']}.html"
    try:
        if not path.exists():
            for attempt in range(3):
                try:
                    with urlopen(Request(entry["upstream_url"], headers={
                        "User-Agent": "Sapid documentation review (read-only)",
                    }), timeout=35) as response:
                        raw = response.read(5 * 1024 * 1024 + 1)
                        if len(raw) > 5 * 1024 * 1024:
                            raise ValueError("official document exceeds audit body budget")
                        result["upstream_resolved_url"] = response.url
                    path.write_bytes(raw)
                    break
                except Exception:
                    if attempt == 2:
                        raise
                    time.sleep(attempt + 1)
        else:
            raw = path.read_bytes()
        source = raw.decode("utf-8")
        parser = Headings()
        parser.feed(source)
        result["upstream_headings"] = parser.items
        result["upstream_sections"] = [item["text"] for item in parser.items if item["level"] == 2]
        result["upstream_sha256"] = hashlib.sha256(raw).hexdigest()
        result["upstream_bytes"] = len(raw)
        version = re.search(r">v(\d+\.\d+\.\d+)<", source)
        result["upstream_version"] = version[1] if version else None
        result["upstream_status"] = "fetched" if result["upstream_sections"] else "review-empty-sections"
    except Exception as error:
        result["upstream_status"] = "error"
        result["upstream_error"] = str(error)
    return result


def main():
    catalogue = []
    for line in (ROOT / "examples/web/src/showcase_catalog.rs").read_text(encoding="utf-8").splitlines():
        if not line.lstrip().startswith("Entry {"):
            continue
        entry = {key: json.loads('"' + value + '"') for key, value in re.findall(
            r'(id|slug|task|name|group): "((?:\\.|[^"\\])*)"', line
        )}
        entry["source"] = re.search(r'code: include_str!\("([^"]+)"\)', line)[1]
        catalogue.append(entry)
    facts = json.loads((ROOT / "docs/mantine-api-audit.json").read_text(encoding="utf-8"))
    normalize = lambda value: re.sub(r"\W", "", value).lower()
    entries = []
    for fact in facts["entries"]:
        matches = [entry for entry in catalogue if normalize(entry["name"]) == normalize(fact["name"])]
        if len(matches) != 1:
            raise ValueError(f"ambiguous shared-page mapping: {fact['name']}")
        entries.append({**matches[0], "upstream_url": fact["url"], "package": fact["package"],
                        "previous_sections": fact["sections"]})
    assert len(entries) == len(catalogue) == 181
    report = {"review_date": datetime.date(2026, 11, 4).isoformat(), "scope": "181 shared documentation pages",
              "entries": []}
    with concurrent.futures.ThreadPoolExecutor(max_workers=6) as pool:
        for index, entry in enumerate(pool.map(fetch, entries), 1):
            report["entries"].append(entry)
            if index % 30 == 0 or entry["upstream_status"] == "error":
                print(f"upstream {index}/181: {entry['slug']} {entry['upstream_status']}", flush=True)
    destination = OUTPUT / "upstream.json"
    destination.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    (OUTPUT / "pages.json").write_text(json.dumps(entries, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"pages": len(entries), "errors": [entry["slug"] for entry in report["entries"]
                      if entry["upstream_status"] == "error"]}))


if __name__ == "__main__":
    main()
