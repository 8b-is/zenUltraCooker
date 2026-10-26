#!/usr/bin/env python3
"""
generate-app-icons.py — End-to-End Professional App Icon & Brand Asset Pack Generator
for zenUltraCooker (v2.2.0).

Generates:
1. Master Vector Icon (`assets/icons/app-icon-master.svg`) & Clean Editorial Brand Hero (`assets/brand-hero.svg`)
2. iOS / iPadOS / macOS Xcode Asset Catalog (`mobile/ios/ZenUltraCookerApp/Assets.xcassets/AppIcon.appiconset/`)
   - All 15 standard PNG sizes (20pt..1024pt) + `Contents.json`
3. Android Mipmap & Adaptive Icon Pack (`mobile/android/app/src/main/res/`)
   - `mipmap-{mdpi,hdpi,xhdpi,xxhdpi,xxxhdpi}/ic_launcher.png` & `ic_launcher_round.png`
   - `drawable/ic_launcher_background.xml`, `drawable/ic_launcher_foreground.xml`, `mipmap-anydpi-v26/ic_launcher.xml`
   - `assets/icons/playstore-icon-512.png`
4. Web / PWA / Favicon Pack (`assets/icons/` + `site.webmanifest`)
   - `favicon.ico` (16, 32, 48), `favicon-16x16.png`, `favicon-32x32.png`, `apple-touch-icon.png` (180x180),
     `icon-192.png`, `icon-512.png`, `icon-maskable-512.png`, `og-social-card.png` (1200x630)
"""

import json
import math
from pathlib import Path
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parent.parent
ICONS_DIR = ROOT / "assets" / "icons"
IOS_APPICONSET = ROOT / "mobile" / "ios" / "ZenUltraCookerApp" / "Assets.xcassets" / "AppIcon.appiconset"
ANDROID_RES = ROOT / "mobile" / "android" / "app" / "src" / "main" / "res"


def write_svg_assets() -> None:
    ICONS_DIR.mkdir(parents=True, exist_ok=True)
    master_svg = """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" width="1024" height="1024">
  <defs>
    <linearGradient id="bgGrad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#1E2923"/>
      <stop offset="55%" stop-color="#16201B"/>
      <stop offset="100%" stop-color="#0F1713"/>
    </linearGradient>
    <linearGradient id="potGrad" x1="0%" y1="0%" x2="0%" y2="100%">
      <stop offset="0%" stop-color="#E06D44"/>
      <stop offset="100%" stop-color="#B84A27"/>
    </linearGradient>
    <linearGradient id="goldGrad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#F6D365"/>
      <stop offset="100%" stop-color="#D49B35"/>
    </linearGradient>
  </defs>

  <!-- Background Canvas -->
  <rect width="1024" height="1024" rx="224" fill="url(#bgGrad)"/>
  <rect x="36" y="36" width="952" height="952" rx="196" fill="none" stroke="#D49B35" stroke-opacity="0.28" stroke-width="6"/>

  <!-- Subtle Warm Hearth Halo -->
  <circle cx="512" cy="540" r="320" fill="#E06D44" fill-opacity="0.10"/>

  <!-- Warm Rising Steam Waves (3 clean culinary curves) -->
  <path d="M392 310 C372 265, 412 230, 392 185" fill="none" stroke="url(#goldGrad)" stroke-width="22" stroke-linecap="round"/>
  <path d="M512 290 C492 240, 532 200, 512 150" fill="none" stroke="url(#goldGrad)" stroke-width="26" stroke-linecap="round"/>
  <path d="M632 310 C612 265, 652 230, 632 185" fill="none" stroke="url(#goldGrad)" stroke-width="22" stroke-linecap="round"/>

  <!-- Dutch Oven / Air Fryer Lid & Gold Knob -->
  <rect x="466" y="344" width="92" height="34" rx="17" fill="url(#goldGrad)"/>
  <path d="M268 418 C268 378, 360 364, 512 364 C664 364, 756 378, 756 418 Z" fill="#F4EFE6"/>

  <!-- Side Handles -->
  <path d="M206 476 H266 V526 H206 C190 526, 180 514, 180 501 C180 488, 190 476, 206 476 Z" fill="#D49B35"/>
  <path d="M758 476 H818 C834 476, 844 488, 844 501 C844 514, 834 526, 818 526 H758 Z" fill="#D49B35"/>

  <!-- Cast-Iron Hearth Vessel Body -->
  <path d="M256 436 H768 L732 692 C718 776, 646 828, 558 828 H466 C378 828, 306 776, 292 692 Z" fill="url(#potGrad)"/>

  <!-- Clean Warm Hearth Flame / Leaf Emblem on Vessel Center -->
  <path d="M512 520 C550 568, 576 606, 576 648 C576 686, 548 716, 512 716 C476 716, 448 686, 448 648 C448 606, 474 568, 512 520 Z" fill="#FFF8EB"/>
  <path d="M512 578 C532 606, 544 628, 544 652 C544 672, 530 688, 512 688 C494 688, 480 672, 480 652 C480 628, 492 606, 512 578 Z" fill="#F6D365"/>

  <!-- Minimalist Base Trivet Line -->
  <rect x="352" y="856" width="320" height="18" rx="9" fill="#4A9E8E" fill-opacity="0.85"/>
</svg>
"""
    (ICONS_DIR / "app-icon-master.svg").write_text(master_svg, encoding="utf-8")

    brand_hero_svg = """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1200 420" width="1200" height="420">
  <defs>
    <linearGradient id="heroBg" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#16201B"/>
      <stop offset="60%" stop-color="#1E2B24"/>
      <stop offset="100%" stop-color="#24332B"/>
    </linearGradient>
    <linearGradient id="vessel" x1="0%" y1="0%" x2="0%" y2="100%">
      <stop offset="0%" stop-color="#E06D44"/>
      <stop offset="100%" stop-color="#B84A27"/>
    </linearGradient>
  </defs>
  <rect width="1200" height="420" rx="28" fill="url(#heroBg)"/>
  <rect x="16" y="16" width="1168" height="388" rx="20" fill="none" stroke="#D49B35" stroke-opacity="0.3" stroke-width="2"/>

  <!-- Left Emblem Card -->
  <g transform="translate(72, 54) scale(0.30)">
    <rect width="1024" height="1024" rx="224" fill="#101814" stroke="#D49B35" stroke-width="12"/>
    <path d="M392 310 C372 265, 412 230, 392 185" fill="none" stroke="#F6D365" stroke-width="24" stroke-linecap="round"/>
    <path d="M512 290 C492 240, 532 200, 512 150" fill="none" stroke="#F6D365" stroke-width="28" stroke-linecap="round"/>
    <path d="M632 310 C612 265, 652 230, 632 185" fill="none" stroke="#F6D365" stroke-width="24" stroke-linecap="round"/>
    <rect x="466" y="344" width="92" height="34" rx="17" fill="#F6D365"/>
    <path d="M268 418 C268 378, 360 364, 512 364 C664 364, 756 378, 756 418 Z" fill="#F4EFE6"/>
    <path d="M256 436 H768 L732 692 C718 776, 646 828, 558 828 H466 C378 828, 306 776, 292 692 Z" fill="url(#vessel)"/>
    <path d="M512 520 C550 568, 576 606, 576 648 C576 686, 548 716, 512 716 C476 716, 448 686, 448 648 C448 606, 474 568, 512 520 Z" fill="#FFF8EB"/>
  </g>

  <!-- Editorial Typography -->
  <text x="424" y="128" fill="#F6D365" font-family="-apple-system, BlinkMacSystemFont, 'Segoe UI', Inter, sans-serif" font-size="16" font-weight="700" letter-spacing="2">FAMILY KITCHEN &amp; HOUSEHOLD PLANNER • v2.2.0</text>
  <text x="424" y="186" fill="#FAF6F0" font-family="Georgia, 'Iowan Old Style', serif" font-size="46" font-weight="700">zenUltraCooker</text>
  <text x="424" y="232" fill="#D5E0DA" font-family="-apple-system, BlinkMacSystemFont, 'Segoe UI', Inter, sans-serif" font-size="21" font-weight="400">Bring the family back to the dinner table — without the weeknight stress.</text>

  <!-- Feature Pills -->
  <g transform="translate(424, 266)">
    <rect x="0" y="0" width="210" height="40" rx="20" fill="#26382F" stroke="#4A9E8E" stroke-width="1.5"/>
    <text x="105" y="25" text-anchor="middle" fill="#FAF6F0" font-family="-apple-system, sans-serif" font-size="14" font-weight="600">US ZIP Grocery Flyers</text>

    <rect x="224" y="0" width="220" height="40" rx="20" fill="#26382F" stroke="#E06D44" stroke-width="1.5"/>
    <text x="334" y="25" text-anchor="middle" fill="#FAF6F0" font-family="-apple-system, sans-serif" font-size="14" font-weight="600">Smart Air Fryer Routing</text>

    <rect x="458" y="0" width="248" height="40" rx="20" fill="#26382F" stroke="#F6D365" stroke-width="1.5"/>
    <text x="582" y="25" text-anchor="middle" fill="#FAF6F0" font-family="-apple-system, sans-serif" font-size="14" font-weight="600">Honest Prep &amp; Cleanup Times</text>
  </g>

  <text x="424" y="346" fill="#9BB0A5" font-family="-apple-system, BlinkMacSystemFont, sans-serif" font-size="15">Fridge / Freezer / Pantry Zero-Waste • Household Essentials • Stripe, Apple Pay &amp; Google Pay • iOS &amp; Android</text>
</svg>
"""
    (ROOT / "assets" / "brand-hero.svg").write_text(brand_hero_svg, encoding="utf-8")


def render_master_icon(size: int = 1024, maskable: bool = False, round_crop: bool = False) -> Image.Image:
    """Renders the crisp high-DPI culinary hearth icon via 2x supersampled Pillow vector drawing."""
    ss = 2
    W = size * ss
    img = Image.new("RGBA", (W, W), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    # Background shape
    bg_color = (22, 32, 27, 255)  # Deep Forest Espresso #16201B
    border_color = (212, 155, 53, 90)  # Warm Brass #D49B35

    if maskable:
        draw.rectangle([0, 0, W, W], fill=bg_color)
    elif round_crop:
        draw.ellipse([0, 0, W - 1, W - 1], fill=bg_color)
        pad = int(W * 0.035)
        draw.ellipse([pad, pad, W - 1 - pad, W - 1 - pad], outline=border_color, width=max(2, int(W * 0.008)))
    else:
        radius = int(W * 0.218)
        draw.rounded_rectangle([0, 0, W - 1, W - 1], radius=radius, fill=bg_color)
        pad = int(W * 0.038)
        draw.rounded_rectangle(
            [pad, pad, W - 1 - pad, W - 1 - pad],
            radius=int(radius * 0.86),
            outline=border_color,
            width=max(2, int(W * 0.006)),
        )

    scale = 0.82 if maskable else 1.0

    def sc(val: float) -> int:
        centered = (val - 512.0) * scale + 512.0
        return int(round(centered * W / 1024.0))

    def sc_len(val: float) -> int:
        return max(1, int(round(val * scale * W / 1024.0)))

    # Subtle warm hearth glow circle
    glow_r = sc_len(290)
    cx, cy = sc(512), sc(548)
    draw.ellipse([cx - glow_r, cy - glow_r, cx + glow_r, cy + glow_r], fill=(224, 109, 68, 28))

    # Steam bars (3 rounded vertical capsules)
    gold = (246, 211, 101, 255)
    cream = (244, 239, 230, 255)
    terracotta = (224, 109, 68, 255)
    terracotta_dark = (184, 74, 39, 255)
    sage = (74, 158, 142, 255)

    steam_w = sc_len(24)
    for sx, top_y, bot_y in [(396, 195, 305), (512, 160, 290), (628, 195, 305)]:
        x0 = sc(sx) - steam_w // 2
        x1 = sc(sx) + steam_w // 2
        draw.rounded_rectangle([x0, sc(top_y), x1, sc(bot_y)], radius=steam_w // 2, fill=gold)

    # Lid knob
    draw.rounded_rectangle([sc(464), sc(336), sc(560), sc(374)], radius=sc_len(18), fill=gold)

    # Lid dome
    draw.rounded_rectangle([sc(268), sc(368), sc(756), sc(420)], radius=sc_len(26), fill=cream)

    # Side handles
    draw.rounded_rectangle([sc(184), sc(472), sc(264), sc(524)], radius=sc_len(22), fill=gold)
    draw.rounded_rectangle([sc(760), sc(472), sc(840), sc(524)], radius=sc_len(22), fill=gold)

    # Main Dutch Oven / Air Fryer Vessel Body
    draw.rounded_rectangle([sc(256), sc(438), sc(768), sc(816)], radius=sc_len(110), fill=terracotta)
    draw.rectangle([sc(256), sc(438), sc(768), sc(560)], fill=terracotta)
    # Subtle lower shadow band on pot
    draw.rounded_rectangle([sc(284), sc(720), sc(740), sc(816)], radius=sc_len(60), fill=terracotta_dark)

    # Center Warm Hearth Flame / Leaf Drop
    flame_outer_r = sc_len(88)
    fcx, fcy = sc(512), sc(642)
    draw.ellipse([fcx - flame_outer_r, fcy - flame_outer_r, fcx + flame_outer_r, fcy + flame_outer_r], fill=cream)
    draw.polygon(
        [(fcx, sc(516)), (fcx - flame_outer_r + sc_len(8), fcy - sc_len(20)), (fcx + flame_outer_r - sc_len(8), fcy - sc_len(20))],
        fill=cream,
    )

    flame_inner_r = sc_len(48)
    ifcy = sc(654)
    draw.ellipse([fcx - flame_inner_r, ifcy - flame_inner_r, fcx + flame_inner_r, ifcy + flame_inner_r], fill=gold)
    draw.polygon(
        [(fcx, sc(580)), (fcx - flame_inner_r + sc_len(4), ifcy - sc_len(10)), (fcx + flame_inner_r - sc_len(4), ifcy - sc_len(10))],
        fill=gold,
    )

    # Base Sage Trivet bar
    draw.rounded_rectangle([sc(348), sc(852), sc(676), sc(874)], radius=sc_len(11), fill=sage)

    return img.resize((size, size), Image.Resampling.LANCZOS)


def generate_ios_appiconset() -> None:
    IOS_APPICONSET.mkdir(parents=True, exist_ok=True)
    ios_specs = [
        ("icon-20.png", "ipad", "20x20", "1x", 20),
        ("icon-20@2x.png", "iphone", "20x20", "2x", 40),
        ("icon-20@3x.png", "iphone", "20x20", "3x", 60),
        ("icon-29.png", "ipad", "29x29", "1x", 29),
        ("icon-29@2x.png", "iphone", "29x29", "2x", 58),
        ("icon-29@3x.png", "iphone", "29x29", "3x", 87),
        ("icon-40.png", "ipad", "40x40", "1x", 40),
        ("icon-40@2x.png", "iphone", "40x40", "2x", 80),
        ("icon-40@3x.png", "iphone", "40x40", "3x", 120),
        ("icon-60@2x.png", "iphone", "60x60", "2x", 120),
        ("icon-60@3x.png", "iphone", "60x60", "3x", 180),
        ("icon-76.png", "ipad", "76x76", "1x", 76),
        ("icon-76@2x.png", "ipad", "76x76", "2x", 152),
        ("icon-83.5@2x.png", "ipad", "83.5x83.5", "2x", 167),
        ("icon-1024.png", "ios-marketing", "1024x1024", "1x", 1024),
    ]
    images_json = []
    for filename, idiom, pt_size, scale, px in ios_specs:
        # iOS icons must be opaque RGB (no alpha channel per App Store validation rules)
        rgba = render_master_icon(size=px, maskable=True, round_crop=False)
        rgb = Image.new("RGB", (px, px), (22, 32, 27))
        rgb.paste(rgba, mask=rgba.split()[3])
        rgb.save(IOS_APPICONSET / filename, format="PNG", optimize=True)
        images_json.append(
            {
                "filename": filename,
                "idiom": idiom,
                "scale": scale,
                "size": pt_size,
            }
        )

    contents = {
        "images": images_json,
        "info": {"author": "xcode", "version": 1},
    }
    (IOS_APPICONSET / "Contents.json").write_text(json.dumps(contents, indent=2) + "\n", encoding="utf-8")


def generate_android_icons() -> None:
    mipmaps = [
        ("mipmap-mdpi", 48),
        ("mipmap-hdpi", 72),
        ("mipmap-xhdpi", 96),
        ("mipmap-xxhdpi", 144),
        ("mipmap-xxxhdpi", 192),
    ]
    for folder, px in mipmaps:
        out_dir = ANDROID_RES / folder
        out_dir.mkdir(parents=True, exist_ok=True)
        sq = render_master_icon(size=px, maskable=False, round_crop=False)
        sq.save(out_dir / "ic_launcher.png", format="PNG", optimize=True)
        rd = render_master_icon(size=px, maskable=False, round_crop=True)
        rd.save(out_dir / "ic_launcher_round.png", format="PNG", optimize=True)

    # Play Store 512x512 icon
    play_512 = render_master_icon(size=512, maskable=False, round_crop=False)
    play_512.save(ICONS_DIR / "playstore-icon-512.png", format="PNG", optimize=True)

    # Adaptive icon vector XMLs
    drawable_dir = ANDROID_RES / "drawable"
    drawable_dir.mkdir(parents=True, exist_ok=True)

    ic_pawshaw_xml = """<?xml version="1.0" encoding="utf-8"?>
<vector xmlns:android="http://schemas.android.com/apk/res/android"
    android:width="108dp"
    android:height="108dp"
    android:viewportWidth="108"
    android:viewportHeight="108">
    <!-- Deep Forest Espresso Background -->
    <path
        android:fillColor="#16201B"
        android:pathData="M0,0h108v108h-108z" />
    <!-- Gold Steam Lines -->
    <path
        android:fillColor="#F6D365"
        android:pathData="M41,23h4v10h-4zM52,19h4v13h-4zM63,23h4v10h-4z" />
    <!-- Lid & Knob -->
    <path
        android:fillColor="#F6D365"
        android:pathData="M48,35h12v4h-12z" />
    <path
        android:fillColor="#F4EFE6"
        android:pathData="M28,39h52v6h-52z" />
    <!-- Terracotta Dutch Oven / Air Fryer Vessel -->
    <path
        android:fillColor="#E06D44"
        android:pathData="M27,47h54l-4,28c-2,7 -8,11 -16,11h-14c-8,0 -14,-4 -16,-11z" />
    <!-- Center Cream Hearth Flame -->
    <path
        android:fillColor="#FFF8EB"
        android:pathData="M54,54l8,12c2,4 -2,10 -8,10s-10,-6 -8,-10z" />
    <!-- Sage Trivet Bar -->
    <path
        android:fillColor="#4A9E8E"
        android:pathData="M36,90h36v3h-36z" />
</vector>
"""
    (drawable_dir / "ic_pawshaw.xml").write_text(ic_pawshaw_xml, encoding="utf-8")


def generate_web_and_pwa_icons() -> None:
    ICONS_DIR.mkdir(parents=True, exist_ok=True)

    icon_16 = render_master_icon(size=16)
    icon_32 = render_master_icon(size=32)
    icon_48 = render_master_icon(size=48)
    icon_180 = render_master_icon(size=180, maskable=True)
    icon_192 = render_master_icon(size=192)
    icon_512 = render_master_icon(size=512)
    icon_maskable_512 = render_master_icon(size=512, maskable=True)

    icon_16.save(ICONS_DIR / "favicon-16x16.png", format="PNG", optimize=True)
    icon_32.save(ICONS_DIR / "favicon-32x32.png", format="PNG", optimize=True)
    icon_180.save(ICONS_DIR / "apple-touch-icon.png", format="PNG", optimize=True)
    icon_192.save(ICONS_DIR / "icon-192.png", format="PNG", optimize=True)
    icon_512.save(ICONS_DIR / "icon-512.png", format="PNG", optimize=True)
    icon_maskable_512.save(ICONS_DIR / "icon-maskable-512.png", format="PNG", optimize=True)

    # Save root favicon.ico (multi-size 16, 32, 48)
    icon_48.save(ROOT / "favicon.ico", format="ICO", sizes=[(16, 16), (32, 32), (48, 48)])
    icon_48.save(ICONS_DIR / "favicon.ico", format="ICO", sizes=[(16, 16), (32, 32), (48, 48)])

    # Generate clean 1200x630 OpenGraph Social Card PNG
    og = Image.new("RGB", (1200, 630), (22, 32, 27))
    draw = ImageDraw.Draw(og)
    draw.rounded_rectangle([28, 28, 1172, 602], radius=32, outline=(212, 155, 53), width=3)
    logo = render_master_icon(size=360)
    og.paste(logo, (88, 135), mask=logo.split()[3])
    # Accent bars representing clean typography layout on social card
    draw.rounded_rectangle([500, 175, 780, 195], radius=10, fill=(246, 211, 101))
    draw.rounded_rectangle([500, 225, 1060, 285], radius=14, fill=(250, 246, 240))
    draw.rounded_rectangle([500, 315, 980, 345], radius=10, fill=(190, 208, 198))
    draw.rounded_rectangle([500, 385, 690, 435], radius=22, fill=(224, 109, 68))
    draw.rounded_rectangle([710, 385, 910, 435], radius=22, fill=(74, 158, 142))
    og.save(ICONS_DIR / "og-social-card.png", format="PNG", optimize=True)

    manifest = {
        "name": "zenUltraCooker — Family Kitchen & Household Planner",
        "short_name": "zenUltraCooker",
        "description": "Smart US ZIP grocery flyers, fridge/freezer/pantry zero-waste tracker, Air Fryer appliance routing, honest prep + cleanup recipes, and family dinner planner.",
        "start_url": "./index.html",
        "display": "standalone",
        "background_color": "#FAF6F0",
        "theme_color": "#16201B",
        "icons": [
            {"src": "assets/icons/favicon-16x16.png", "sizes": "16x16", "type": "image/png"},
            {"src": "assets/icons/favicon-32x32.png", "sizes": "32x32", "type": "image/png"},
            {"src": "assets/icons/apple-touch-icon.png", "sizes": "180x180", "type": "image/png"},
            {"src": "assets/icons/icon-192.png", "sizes": "192x192", "type": "image/png", "purpose": "any"},
            {"src": "assets/icons/icon-512.png", "sizes": "512x512", "type": "image/png", "purpose": "any"},
            {"src": "assets/icons/icon-maskable-512.png", "sizes": "512x512", "type": "image/png", "purpose": "maskable"},
        ],
    }
    (ROOT / "site.webmanifest").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    write_svg_assets()
    generate_ios_appiconset()
    generate_android_icons()
    generate_web_and_pwa_icons()
    print("[icons] Successfully generated E2E App Icon Packs (iOS AppIcon.appiconset, Android mipmap-*, Web/PWA & SVG master).")
