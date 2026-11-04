"""Assemble review-only screenshot contact sheets; keep original captures intact."""
import json
import math
import os
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(os.environ.get("SAPID_BROWSER_OUTPUT", "D:/BuildCache/sapid/shared-page-audit"))
PAGES = json.loads((ROOT / "pages.json").read_text(encoding="utf-8"))
LOCAL = {page["slug"]: page for page in json.loads((ROOT / "local-pages.json").read_text(encoding="utf-8"))["pages"]}
UPSTREAM = {page["slug"]: page for page in json.loads((ROOT / "upstream.json").read_text(encoding="utf-8"))["entries"]}
FONT = ImageFont.truetype("C:/Windows/Fonts/segoeui.ttf", 17)
SMALL = ImageFont.truetype("C:/Windows/Fonts/segoeui.ttf", 13)
DESTINATION = ROOT / "contact-sheets"
DESTINATION.mkdir(exist_ok=True)


def paste(canvas, source, x, y, width, height):
    draw = ImageDraw.Draw(canvas)
    if not source.exists():
        draw.text((x + 8, y + 20), "Standard preview not captured", font=SMALL, fill="#777777")
        return
    with Image.open(source) as original:
        image = original.convert("RGB")
        # The complete original remains alongside this explicitly cropped index.
        image = image.crop((0, 0, image.width, min(image.height, 620)))
        image.thumbnail((width, height))
        canvas.paste(image, (x + (width - image.width) // 2, y))
        draw.text((x + 5, y + height - 18), f"{original.width} x {original.height}; top crop", font=SMALL, fill="#777777")


for start in range(0, len(PAGES), 12):
    canvas = Image.new("RGB", (1720, 2040), "#eef0f2")
    draw = ImageDraw.Draw(canvas)
    for offset, page in enumerate(PAGES[start:start + 12]):
        x, y = (offset % 2) * 860, (offset // 2) * 340
        draw.rectangle((x + 3, y + 3, x + 857, y + 337), fill="white")
        local = LOCAL.get(page["slug"], {}).get("modes", {}).get("desktop", {})
        count = len(UPSTREAM[page["slug"]].get("upstream_sections", []))
        label = f"{start+offset+1:03} {page['name']} | upstream sections {count}; local detail blocks {len(local.get('details',[]))}"
        draw.text((x + 10, y + 6), label, font=FONT, fill="#111111")
        draw.text((x + 10, y + 33), "MANTINE", font=SMALL, fill="#1564ad")
        draw.text((x + 438, y + 33), "SAPID", font=SMALL, fill="#19833b")
        paste(canvas, ROOT / "upstream-previews" / f"{page['slug']}.png", x + 8, y + 55, 416, 275)
        paste(canvas, ROOT / "screenshots" / f"{page['slug']}-desktop.png", x + 436, y + 55, 416, 275)
    canvas.save(DESTINATION / f"paired-{start//12+1:02}.jpg", quality=92)

for mode in ["mobile", "dark"]:
    for start in range(0, len(PAGES), 24):
        canvas = Image.new("RGB", (1600, 1860), "#eef0f2")
        draw = ImageDraw.Draw(canvas)
        for offset, page in enumerate(PAGES[start:start + 24]):
            x, y = (offset % 4) * 400, (offset // 4) * 310
            draw.rectangle((x + 3, y + 3, x + 397, y + 307), fill="white")
            values = LOCAL.get(page["slug"], {}).get("modes", {}).get(mode, {})
            draw.text((x + 10, y + 5), f"{start+offset+1:03} {page['name']}", font=FONT, fill="#111111")
            draw.text((x + 10, y + 31), f"{mode}; preview overflow {values.get('previewOverflow','?')}px", font=SMALL, fill="#a52121" if values.get("previewOverflow",0) else "#777777")
            paste(canvas, ROOT / "screenshots" / f"{page['slug']}-{mode}.png", x + 8, y + 51, 384, 250)
        canvas.save(DESTINATION / f"{mode}-{start//24+1:02}.jpg", quality=92)
print(f"Prepared {math.ceil(len(PAGES)/12)} paired sheets and {2*math.ceil(len(PAGES)/24)} local mode sheets")
