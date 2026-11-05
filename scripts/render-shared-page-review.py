"""Render the dated shared-page review from its reviewed, offline JSON record."""
import argparse
from collections import Counter
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "docs/shared-page-review-2026-11-04.json"
DESTINATION = SOURCE.with_suffix(".md")


def render(data):
    pages = data["pages"]
    assert len(pages) == len({page["slug"] for page in pages}) == 181
    assert all(page["priority"] in {"P1", "P2", "P3"} for page in pages)
    counts = Counter(page["priority"] for page in pages)
    lines = [
        "# Sapid / Mantine 共同页面审查（2026-11-04）", "",
        data["conclusion"], "",
        "## 范围与证据", "",
        *[f"- {item}" for item in data["scope"]], "",
        "优先级表示后续工作的顺序：P1 为已复现的布局问题、错用示例或已核实的核心功能差异；P2 为文档、呈现与功能展示改进；P3 为本轮未发现急迫问题、仍建议补充的说明。它不是平台支持等级。", "",
        f"逐页记录：P1 {counts['P1']} 页，P2 {counts['P2']} 页，P3 {counts['P3']} 页。一个页面可同时涉及多个问题；下列问题按根因归并。", "",
        "## 应优先处理的问题", "",
    ]
    for finding in data["findings"]:
        lines.extend([f"### {finding['id']} · {finding['title']}", "", finding["evidence"], "",
                      f"改进：{finding['recommendation']}", "", f"验收：{finding['acceptance']}", ""])
        if finding.get("sources"):
            lines.extend(["位置：" + "；".join(f"[{item['label']}]({item['path']})" for item in finding["sources"]) + "。", ""])
    lines.extend(["## 181 页逐页清单", "",
                  "每行记录本轮实际看到的问题或建议，功能未充分操作验证的项目使用“展示/说明/补测”，不据此判定 API 缺失。上游链接是本轮实际访问的对应官方页面。Sapid 对应 Rust 示例路径、页面结构计数及截图路径记录在同名 JSON 中。", ""])
    group = None
    for index, page in enumerate(pages, 1):
        if page["group"] != group:
            group = page["group"]
            lines.extend([f"### {group}", "", "| # | 页面 / 官方参考 | 优先级 · 类型 | 观察与下一步 |", "| --- | --- | --- | --- |"])
        note = page["recommendation"].replace("|", "\\|")
        lines.append(f"| {index} | [{page['name']}]({page['upstream_url']}) | {page['priority']} · {page['category']} | {note} |")
        if index == len(pages) or pages[index]["group"] != group:
            lines.append("")
    lines.extend(["## 建议执行顺序", "", *[f"{i}. {item}" for i, item in enumerate(data["next_steps"], 1)], "",
                  "## 复核与局限", "", *[f"- {item}" for item in data["limitations"]], "",
                  "## 复现", "", "原始 HTML、完整截图、联系表和量测 JSON 均保存在 `project build directory/sapid/shared-page-audit`，不进入运行时依赖或发布资源。", "",
                  "```powershell", "$env:SAPID_BUILD_ROOT = 'project build directory/sapid'", "$env:SAPID_BROWSER_OUTPUT = 'project build directory/sapid/shared-page-audit'",
                  "$env:SAPID_PLAYWRIGHT = 'project build directory/sapid/node-tools/node_modules/playwright'", "$env:PLAYWRIGHT_BROWSERS_PATH = 'project build directory/sapid/browsers'",
                  "$env:TEMP = 'project build directory/sapid/tmp'", "$env:TMP = $env:TEMP", "$env:PYTHONPYCACHEPREFIX = 'project build directory/sapid/pycache'",
                  "# 先启动本轮发布资源的 8795 预览服务器，再执行：", "python scripts/audit-shared-pages.py", "node scripts/audit-shared-pages.cjs",
                  "node scripts/audit-shared-pages.cjs --headings-only", "node scripts/audit-upstream-previews.cjs", "node scripts/audit-shared-page-probes.cjs",
                  "python scripts/audit-page-contact-sheets.py", "# 人工逐页审查并更新 JSON 后，重新渲染报告：", "python scripts/render-shared-page-review.py",
                  "python scripts/render-shared-page-review.py --check", "```", "",
                  "抓取器会复用本轮缓存目录中已有 HTML；需要复核未来版本时应通过 SAPID_BROWSER_OUTPUT 使用新的日期目录并新建报告，不把缓存命中当作新抓取。截图脚本只采集证据，不会自动给出功能等价结论。", ""])
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    data = json.loads(SOURCE.read_text(encoding="utf-8"))
    text = render(data)
    if args.check:
        if DESTINATION.read_text(encoding="utf-8") != text:
            raise SystemExit("Shared-page review differs from JSON; run the renderer.")
        print("181 unique page records; Markdown matches reviewed JSON.")
    else:
        DESTINATION.write_text(text, encoding="utf-8")
        print(DESTINATION)


if __name__ == "__main__":
    main()
