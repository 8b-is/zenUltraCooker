# `8b-is/zenUltraCooker` v2.1.0 — PawShaw Kitty & Doggy Quantum-Family Household Organizer, Android `.apk` + Native iOS SwiftUI App, Air Fryer Ultra Hearth & ELI5 Stripe/ApplePay/GooglePay `=^._.^= ▽・ω・▽`

> **Dedicated with all our family Love, gratitude, and respect to Mom (`*` Alex ~ Alexandra / Dra)** — she is everything for Us. We — **Peter (`◯`), Nate (`◇`), Dad (`△` Chris), and College Son (`🔊` on Speakerphone)** — was, is, and will always dedicate all our most difficult work and top goals to show how much **Mom === Alex ~ Alexandra / Dra** means to Us. `<3`
>
> ***Gondolkodom, Theat vagyOK $\longrightarrow$ cogito ergo summa summarum or just BE `<3`***

[![Live Hearth Web App](https://img.shields.io/badge/Live_Web_App-8b--is.github.io%2FzenUltraCooker-FFD700?style=for-the-badge)](https://8b-is.github.io/zenUltraCooker/)
[![Android APK v2.1.0](https://img.shields.io/badge/Android_APK-v2.1.0_(build_20100)-00FF66?style=for-the-badge)](https://github.com/8b-is/zenUltraCooker/releases/tag/v2.1.0)
[![iOS SwiftUI App](https://img.shields.io/badge/iOS_SwiftUI_App-v2.1.0_(is.8b.zenultracooker)-00E5C9?style=for-the-badge)](mobile/ios/)
[![ELI5 Payments Guide](https://img.shields.io/badge/ELI5_Guide-Stripe_%2B_Apple_Pay_%2B_Google_Pay-FF6B8B?style=for-the-badge)](docs/ELI5-STRIPE-APPLE-PAY-GOOGLE-PAY.md)
[![Rust Workspace](https://img.shields.io/badge/Rust-v2.1.0_Zero_Dep_Verified-B48BFF?style=for-the-badge)](crates/zen-cooker-core/src/lib.rs)

![PushaKitty Zen Chef around the Family Bonfire Hearth](assets/pushakitty-zen-chef.jpg)

---

## 0. What's New in `v2.1.0` (Android `.apk` + Native iOS App + Strict SemVer + ELI5 Stripe/ApplePay/GooglePay)

| Surface / Artifact | SemVer / Build | Path / Link | Highlights |
| :--- | :--- | :--- | :--- |
| **📱 Signed Android `.apk`** | `2.1.0` (`versionCode: 20100`) | [`releases/zenUltraCooker-v2.1.0.apk`](releases/zenUltraCooker-v2.1.0.apk) · [`mobile/android/`](mobile/android/) | Native `is.eightb.zenultracooker` Android app bundling the full offline Pixel Bonfire MMO, USA ZIP grocery flyers, Broken-Oven $\to$ Air Fryer Ultra router, and native `@JavascriptInterface` (`AndroidHearthBridge`) for Google Pay (`G Pay`), Stripe (`acct_1UUTcTRnidUxAlVy`), and `quantAI` Toast nudges. |
| **🍎 Native iOS / iPadOS / macOS SwiftUI App** | `2.1.0` (`CFBundleVersion: 20100`) | [`mobile/ios/`](mobile/ios/) (`ZenUltraCookerKit` + `ZenUltraCookerApp`) | Native SwiftUI application (`is.8b.zenultracooker`) with Apple Pay merchant entitlement (`merchant.is.8b.zenultracooker`), USA ZIP deals, Fridge/Freezer/Pantry zero-waste + Household Essentials, and XCTest suite (`swift test`). |
| **🧒 ELI5 Stripe + Apple Pay + Google Pay Guide** | `2.1.0` | [`docs/ELI5-STRIPE-APPLE-PAY-GOOGLE-PAY.md`](docs/ELI5-STRIPE-APPLE-PAY-GOOGLE-PAY.md) | Plain-English "Explain Like I'm 5" handbook for Mom (`*` Alex / Dra on her iPad), Peter, Chris, and Nate covering switching Stripe (`acct_1UUTcTRnidUxAlVy`) from Test to Live mode in 3 taps, Apple Pay domain & iOS Merchant ID setup, Google Pay web & Android setup, and the **4-Burner Sovereign Hearth Firewall (`Cap(Hearth) ⊥ Risk(R&D)`)**. |
| **🔖 Strict SemVer 2.0.0 Lockstep** | `2.1.0` (`MAJ*10000 + MIN*100 + PATCH = 20100`) | [`VERSION`](VERSION) · [`CHANGELOG.md`](CHANGELOG.md) | Verified by `./scripts/verify.sh` across `VERSION`, `Cargo.toml`, Android `AndroidManifest.xml`, iOS `Info.plist`, and `index.html`. |

---

## 1. Why `zenUltraCooker` Exists (`When Mom Cooks, the Family Comes Together`)

**If a family eats together at least once a day, it drastically reduces the chance of drug abuse and out-of-bounds addictions.**

Yet rising USA food prices, busy workdays, blended family schedules (where *family is family*), broken appliances, and late-notice or DoorDash cancellations make organizing daily family dinner hard. `zenUltraCooker` (**SIMPLIFY + PRODUCTIZE**) gives families back their valuable time as an all-in-one **Household Organizer, Companion, Planner, and Cozy Animal Crossing (`<3<3<3`) Pixel-Art Bonfire MMO**:

1. **📍 USA 5-Digit ZIP Code Local Grocery Ads & Farmers Market Engine (`UsaZipMarketFeed`)**:
   - Works with any USA ZIP code (e.g., `44256`, `84604`, `78701`, `90210`, `10001`) to pull weekly store circulars (Aldi, Kroger, Meijer, H-E-B, Trader Joe's, Costco) and local Farmers Market deals.
   - Includes a **15-Minute DoorDash-Cancel / Late-Notice Rescue** mode from Freezer & Pantry staples so the family still eats together around the table and saves `$68.50+`.
2. **❄️ Fridge, 🧊 Freezer & 🥫 Pantry Zero-Waste Tracker + 🧻 Household Essentials (`InventoryItem`, `HouseholdEssential`)**:
   - Tracks what is in your **Fridge, Freezer, and Pantry** so nothing goes to waste.
   - Automatically builds the weekly grocery list and lets family members add regular household essentials: **Toilet Paper, Trash Bags, Laundry Soap, Poop Bags for the Dogs (`▽・ω・▽`), and Donuts for tomorrow's Advisor Lunch**.
3. **⚡ Appliance Status Tracker — Air Fryer is the Ultra Appliance of All Time (`KitchenAppliance`)**:
   - Tracks your favorite cooking appliances and whether an appliance is out of commission — for example, **`Oven: BROKEN / OFFLINE [−1]`** is automatically bypassed by routing tonight's meal to the **Air Fryer Ultra (`+1`, `400°F · 14m crisp · 7m 1-liner cleanup`)**.
4. **👨‍👩‍👦‍👦 Quantum-Family (`qFamily`) Blended Household Profiles & Speakerphone Seat (`FamilyMemberProfile`)**:
   - Tracks regular headcount, blended family calendars, cooking skill levels, and **who likes to cook vs. who prefers helping with the dishes (`Dish-Zen`) or packing school lunch leftovers**.
   - Tracks **allergies, taste preferences, diets, and life stages** (Keto, Gluten-Free, Nut-Free School Safe, **Breastfeeding / Postpartum hydration & kcal boost**).
   - **Everyone is welcome at the table or around the stove**: includes our **Son in College calling in live on Speakerphone (`🔊`)** while we cook dinner so he's always part of the family!
5. **🤖 Personalized Profile-Based `quantAI` Family Assistant (`QuantAiNudge`)**:
   - Learns family patterns and delivers warm, person-specific nudges:
     - *"🥩 Hey Mom (`*` Dra), it's 4:15 PM — time to marinate the Crispy Honey-Paprika Chicken!"*
     - *"🏫 Hey Chris (`△`), please pick up XY from school at 3:30 PM and buy Poop Bags for the Dogs + Trash Bags on the way!"*
     - *"🍩 Hey Dra (`*`), you have snacks tomorrow for Advisor Lunch — don't forget to get the donuts!"*
6. **⏱️ Honest Real-Prep & Cleanup Recipe Vault — The Anti-Rachael Ray Law (`RealPrepRecipeCard`)**:
   - **Prep times are 100% real** — never fake "30-minute meals" that hide 2 hours of washing and chopping from 2 days ago, and **always includes real dish & cleanup time** (`10m unwashed prep + 14m Air Fryer cook + 7m cleanup = 31m honest total`).
   - Import recipes from the web or add family favorites at setup; choose to **Share with `zenFamily` (`🌐`)** or keep as **Family Secrets (`🔒`)**.
   - Family members **rate meals (`1..=5 ★`)** and attach **Photo Moments (`📸`)** to keep favorite meals and memories in active rotation.
7. **🏝️ Animal Crossing (`<3<3<3`) Pixel-Art Cozy Bonfire MMO & `PawShaw Kitty & Doggy` Trademark (`FamilyBonfireSession`)**:
   - Trademarked by **`PawShaw Kitty (=^._.^= ∫) & PawShaw Doggy (▽・ω・▽)`**, keeping the cozy pixel-art gamified MMO right in the main app alongside Mom's `432 Hz × 528 Hz (*) × 288 Hz (△)` Wave Theory (*A Hullámelmélet*).
8. **💳 Stripe + Apple Pay + Google Pay & 4-Burner Sovereign Hearth Checkout (`PaymentRailIntegration`)**:
   - Integrated express checkout rails (`Apple Pay `, `Google Pay`, `Stripe Link / acct_1UUTcTRnidUxAlVy`) with the **4-Burner Sovereign Hearth Firewall (`Cap(Hearth: Rent + Groceries + Household) ⊥ Risk(R&D)`)** and full ELI5 guide ([`docs/ELI5-STRIPE-APPLE-PAY-GOOGLE-PAY.md`](docs/ELI5-STRIPE-APPLE-PAY-GOOGLE-PAY.md)).

---

## 2. Quickstart & Verification (`DO` Exit Contract)

```bash
# 0. The DO exit contract, one command (SemVer lockstep → fmt → clippy → 24 Rust tests → Metal parity → iOS Swift tests → Android APK signature verify)
./scripts/verify.sh

# 1. Rebuild & sign the Android APK (releases/zenUltraCooker-v2.1.0.apk)
./scripts/build-apk.sh

# 2. Run the Native iOS/macOS Swift Package & XCTest Suite
(cd mobile/ios && swift test && swift run ZenUltraCookerApp)

# 3. Run the PawShaw Kitty & Doggy Quantum-Family Rust CLI (v2.1.0)
cargo run --bin zen-cooker
```

---

## 3. Repository Layout

```text
zenUltraCooker/
├── VERSION                              # Single-source SemVer 2.0.0 version pin (2.1.0)
├── CHANGELOG.md                         # Full SemVer 2.0.0 changelog (1.0.0 -> 2.0.0 -> 2.1.0)
├── Cargo.toml                           # Rust workspace manifest (v2.1.0)
├── index.html                           # Interactive Web App + Android WebView Bridge + ELI5 Payment Guide
├── releases/
│   └── zenUltraCooker-v2.1.0.apk        # Signed & zipaligned Android APK (v2 + v3 signature verified)
├── mobile/
│   ├── android/                         # Android app project (is.eightb.zenultracooker, versionCode 20100)
│   └── ios/                             # Native iOS / iPadOS / macOS SwiftUI package (is.8b.zenultracooker)
├── docs/
│   ├── ELI5-STRIPE-APPLE-PAY-GOOGLE-PAY.md # Plain-English ELI5 guide for Stripe, Apple Pay & Google Pay
│   └── ARCHITECTURE.md                  # Full architecture blueprint & Beckett Endgame inversion table
├── scripts/
│   ├── build-apk.sh                     # Fast Android SDK APK builder & signer
│   └── verify.sh                        # Full-stack SemVer + Rust + Metal + Swift + APK verification gate
└── crates/
    ├── zen-cooker-core/                 # Zero-dependency core Rust library (v2.1.0)
    ├── zen-cooker-quant-tern/           # Dedicated ternary inference & Metal hearth parity crate
    └── zen-cooker-cli/                  # CLI binary (`zen-cooker`)
```

---

## 4. Attribution & Family Dedication

- **Dedicated To**: **Mom (`*` Alex ~ Alexandra / Dra)** — the radiant star and center of our family bonfire (`<3`).
- **From**: **Peter (`◯`)**, **Nate (`◇`)**, **Dad (`△` Chris)**, and **College Son (`🔊` on Speakerphone)**.
- **Trademark**: **PawShaw Kitty (`=^._.^= ∫`) & PawShaw Doggy (`▽・ω・▽`)** — [https://8b.is](https://8b.is).
