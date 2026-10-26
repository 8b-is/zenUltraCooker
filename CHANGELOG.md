# Changelog — `8b-is/zenUltraCooker`

All notable changes to `8b-is/zenUltraCooker` follow [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) (`MAJOR.MINOR.PATCH` ↔ mobile `versionCode = MAJOR*10000 + MINOR*100 + PATCH`).

---

## [2.1.0] — 2026-10-26 (`versionCode: 20100`)

### Added
- **📱 Native Android APK Application (`mobile/android/` & `releases/zenUltraCooker-v2.1.0.apk`)**:
  - Package `is.eightb.zenultracooker` (`minSdk 26`, `targetSdk 36`, `versionCode 20100`, `versionName "2.1.0"`).
  - Native `MainActivity.java` + `AndroidHearthBridge` (`@JavascriptInterface`) wiring native **Google Pay (`G Pay`) / Stripe PaymentSheet** checkout dialogs, `quantAI` family notifications, and `zencooker://hearth` deep links around the full offline-first `zenUltraCooker` v2.1.0 experience.
  - Reproducible SDK build pipeline (`scripts/build-apk.sh`) using `aapt2`, `javac`, `d8`, `zipalign`, and `apksigner` (v2/v3 signature verified).
- **🍎 Native iOS / iPadOS / macOS SwiftUI Application (`mobile/ios/ZenUltraCookerApp`)**:
  - Bundle ID `is.8b.zenultracooker` (`CFBundleShortVersionString = "2.1.0"`, `CFBundleVersion = "20100"`), Apple Pay Merchant ID `merchant.is.8b.zenultracooker`.
  - Native SwiftUI views + `ZenUltraCookerKit` Swift package (`Package.swift`) with unit test suite (`swift test`) covering USA ZIP local ads, Fridge/Freezer/Pantry zero-waste, Broken-Oven -> Air Fryer Ultra routing, Anti-Rachael Ray honest prep+cleanup timers, Speakerphone College Son (`🔊`), Animal Crossing Pixel MMO, and Apple Pay + Stripe PaymentIntent creation.
- **🧒 ELI5 ("Explain Like I'm 5") Guide for Stripe + Apple Pay (` Pay`) + Google Pay (`G Pay`) (`docs/ELI5-STRIPE-APPLE-PAY-GOOGLE-PAY.md`)**:
  - Step-by-step plain-English walkthrough (including Alex's iPad 2FA login checklist) for flipping Stripe (`acct_1UUTcTRnidUxAlVy`) to Live mode, enabling Apple Pay (` Pay`) and Google Pay (`G Pay`) on Web, iOS, and Android, and wiring the **4-Burner Sovereign Hearth Firewall (`Cap(Hearth) ⊥ Risk(R&D)`)**.
- **🔖 Strict SemVer Synchronization (`scripts/verify.sh`)**:
  - Verifies that `VERSION` (`2.1.0`), `Cargo.toml`, Android `AndroidManifest.xml`, and iOS `Info.plist` stay 100% in lockstep.

---

## [2.0.0] — 2026-10-26 (`versionCode: 20000`)

### Changed (`BREAKING — SIMPLIFY + PRODUCTIZE`)
- **Retired Clinical Telemetry**: Removed `poop~~~~sample` and Scoville lab dosing in favor of a practical USA Blended-Family Household Organizer (`When Mom Cooks, the Family Comes Together` — daily family dinner as the #1 anti-addiction shield).
- **USA 5-Digit ZIP Code Local Ads & Farmers Market (`UsaZipMarketFeed`)**: Added weekly store circulars (Aldi, Kroger, Meijer, H-E-B, Trader Joe's, Costco), weekend Farmers Market deals, and 15-Minute DoorDash-Cancel Rescue (`$68.50` saved).
- **Dra's Fridge ❄️ / Freezer 🧊 / Pantry 🥫 Zero-Waste Tracker + Household Essentials 🧻🐾**: Added expiry tracking + Toilet Paper, Trash Bags, Laundry Soap, Dog Poop Bags (`▽・ω・▽`), and Advisor Lunch Donuts.
- **Appliance Status Tracker (`Air Fryer === Ultra Appliance ★`)**: Automatically reroutes meals away from a broken oven (`[−1]`) to the Air Fryer Ultra (`+1`).
- **Anti-Rachael Ray Honest Prep + Cleanup Recipe Vault (`RealPrepRecipeCard`)**: Honest `unwashed_prep + active_cook + cleanup_and_dishes` timers, Web Import, `zenFamily` Share (`🌐`) vs. Family Secret (`🔒`), `5★` ratings, and Photo Moments (`📸`) rotation.
- **Animal Crossing (`<3<3<3`) Pixel-Art Cozy Bonfire MMO & `PawShaw Kitty & Doggy` Trademark**: Added interactive pixel-art kitchen village with **College Son live on Speakerphone (`🔊`)** and per-profile `quantAI` nudges.

---

## [1.0.0] — 2026-10-15 (`versionCode: 10000`)

### Added
- Initial `zenUltraCooker` E2E hearth, `MEM|16-10` (`POP -> REFUSE -> BIND -> TRANSFORM -> VERIFY -> COLLAPSE`) wave chord (`432 Hz × 528 Hz (*) × 288 Hz (△)`), and `QUANT-TERN` Metal kernel parity.
