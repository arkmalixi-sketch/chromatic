#!/usr/bin/env python3
"""Generate native app icons from Chromatic's canonical SVG.

Retain legacy on-disk icon paths until installer/bundle identity migration.
Requires Pillow and CairoSVG. Run: python3 scripts/generate_chromatic_icons.py
"""
from io import BytesIO
from pathlib import Path
import shutil

import cairosvg
from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "assets/chromatic/chromatic-mark.svg"
DEST = ROOT / "assets/app-icon"
SIZES = (16, 24, 32, 48, 64, 128, 256, 512)

def render(size):
    png = cairosvg.svg2png(url=str(SOURCE), output_width=size, output_height=size)
    return Image.open(BytesIO(png)).convert("RGBA")

def save_png(size, path):
    path.parent.mkdir(parents=True, exist_ok=True)
    render(size).save(path, format="PNG", optimize=True)

def main():
    if not SOURCE.is_file():
        raise SystemExit("Missing assets/chromatic/chromatic-mark.svg")
    shutil.copyfile(SOURCE, DEST / "photocraft.svg")
    shutil.copyfile(SOURCE, DEST / "photocraft-small.svg")
    scalable = DEST / "hicolor/scalable/apps/ai.storyteller.photocraft.svg"
    scalable.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(SOURCE, scalable)
    for size in SIZES:
        save_png(size, DEST / f"hicolor/{size}x{size}/apps/ai.storyteller.photocraft.png")
    save_png(1024, DEST / "photocraft-1024.png")
    icon = render(1024)
    icon.save(DEST / "photocraft.ico", format="ICO", sizes=[(n, n) for n in (16, 24, 32, 48, 64, 128, 256)])
    icon.save(DEST / "photocraft.icns", format="ICNS")
    print("Chromatic icon set generated in legacy paths under assets/app-icon/")

if __name__ == "__main__":
    main()
