# Changelog

All notable changes to **`zenUltraCooker`** are documented in this file.
This project adheres strictly to [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html).

| Surface | SemVer | Integer Build / Version Code | Package / Bundle Identifier |
| :--- | :--- | :--- | :--- |
| **Workspace (`VERSION` / `Cargo.toml`)** | `2.2.0` | `20200` | `zen-cooker-core` / `zen-cooker-cli` / `zen-cooker-quant-tern` |
| **Android APK (`mobile/android`)** | `2.2.0` | `versionCode = 20200` | `is.eightb.zenultracooker` |
| **iOS / iPadOS App (`mobile/ios`)** | `2.2.0` | `CFBundleVersion = 20200` | `is.8b.zenultracooker` |
| **Web / PWA (`index.html`)** | `2.2.0` | `20200` | `https://8b-is.github.io/zenUltraCooker/` |

---

## [2.2.0] - 2026-10-26

### Added
- **End-to-End App Icon & Brand Asset Packs (`scripts/generate-app-icons.py`)**:
  - **Master Vector Icon & Editorial Hero**: Created [`assets/icons/app-icon-master.svg`](assets/icons/app-icon-master.svg) and [`assets/brand-hero.svg`](assets/brand-hero.svg) featuring a clean, warm terracotta Dutch-oven / Air-Fryer hearth emblem on a deep forest-espresso canvas.
  - **iOS / iPadOS / macOS `AppIcon.appiconset`**: Generated all 15 standard Xcode Asset Catalog PNG sizes (`20pt` through `1024×1024` App Store icon) plus `Contents.json` under `mobile/ios/ZenUltraCookerApp/Assets.xcassets/AppIcon.appiconset/`.
  - **Android Mipmap & Adaptive Icon Suite**: Generated `ic_launcher.png` and `ic_launcher_round.png` across `mipmap-mdpi` (`48px`), `mipmap-hdpi` (`72px`), `mipmap-xhdpi` (`96px`), `mipmap-xxhdpi` (`144px`), and `mipmap-xxxhdpi` (`192px`), plus `assets/icons/playstore-icon-512.png` (`512×512`) and vector `drawable/ic_pawshaw.xml`.
  - **Web / PWA / Favicon Pack**: Generated multi-size `favicon.ico`, `favicon-16x16.png`, `favicon-32x32.png`, `apple-touch-icon.png` (`180×180`), `icon-192.png`, `icon-512.png`, `icon-maskable-512.png`, `og-social-card.png` (`1200×630`), and `site.webmanifest`.

### Changed
- **Professional Editorial Redesign (`index.html` & `README.md`)**:
  - Replaced anime/cyberpunk visuals and internal jargon with a warm, editorial consumer kitchen aesthetic (`#FAF6F0` alabaster cream, `#16201B` forest espresso, `#D95D39` terracotta hearth, `#2A7B6C` sage green).
  - Rebuilt and signed `releases/zenUltraCooker-v2.2.0.apk` (`versionCode: 20200`) with full `mipmap-*` launcher icons and a lightweight offline asset footprint (`< 150 KB`).

---

## [2.1.0] - 2026-10-26

### Added
- **Native Android Application & Signed `.apk` (`mobile/android/`, `releases/zenUltraCooker-v2.1.0.apk`)**:
  - Added full Android project (`package="is.eightb.zenultracooker"`, `minSdk=26`, `targetSdk=36`) with native `@JavascriptInterface` bridge (`AndroidHearthBridge`) for Google Pay (`G Pay`), Apple Pay / Stripe Link checkout, and household reminder toasts.
- **Native iOS / iPadOS / macOS SwiftUI Application (`mobile/ios/`)**:
  - Added Swift Package (`ZenUltraCookerKit` + `ZenUltraCookerApp`) with Apple Pay merchant entitlement (`merchant.is.8b.zenultracooker`) and automated self-tests.
- **Plain-English (ELI5) Stripe, Apple Pay & Google Pay Guide (`docs/ELI5-STRIPE-APPLE-PAY-GOOGLE-PAY.md`)**:
  - Step-by-step setup documentation for Stripe (`acct_1UUTcTRnidUxAlVy`), Apple Pay, Google Pay, and the 4-Burner Household Budget Firewall.

---

## [2.0.0] - 2026-10-26

### Changed
- **Productized Family Kitchen & Household Planner**:
  - Added US 5-digit ZIP code grocery flyer & Farmers Market comparison engine, 15-minute DoorDash-cancel freezer rescue, Fridge/Freezer/Pantry zero-waste expiry tracking, household essentials tracking, broken-oven auto-routing to the Air Fryer, honest prep + cook + dish cleanup recipe vault, blended family profiles with a live speakerphone seat for a son in college, and 1-tap Stripe / Apple Pay / Google Pay checkout.
