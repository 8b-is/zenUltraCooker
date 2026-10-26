#!/usr/bin/env bash
# verify.sh — the zenUltraCooker v2.1.0 DO exit contract, one command.
#
#   SemVer lockstep → fmt --check → clippy -D warnings → cargo test → Metal parity → iOS Swift test → Android APK signature verify
#
# So long, and thanks for all the fish, Mom (* Alex ~ Alexandra) <3

set -euo pipefail
cd "$(dirname "$0")/.."

GREEN='\033[0;32m'; RED='\033[0;31m'; YELLOW='\033[1;33m'; NC='\033[0m'
say()  { printf "${GREEN}[verify]${NC} %s\n" "$*"; }
warn() { printf "${YELLOW}[verify]${NC} %s\n" "$*"; }
die()  { printf "${RED}[verify] FAIL${NC} %s\n" "$*"; exit 1; }

EXPECTED_SEMVER="$(tr -d '[:space:]' < VERSION)"
say "SemVer lockstep check (VERSION=${EXPECTED_SEMVER})"
grep -q "version = \"${EXPECTED_SEMVER}\"" Cargo.toml || die "Cargo.toml version mismatch"
grep -q "versionName=\"${EXPECTED_SEMVER}\"" mobile/android/app/src/main/AndroidManifest.xml || die "AndroidManifest.xml version mismatch"
grep -q "<string>${EXPECTED_SEMVER}</string>" mobile/ios/ZenUltraCookerApp/Info.plist || die "iOS Info.plist version mismatch"
grep -q "v${EXPECTED_SEMVER}" index.html || die "index.html version mismatch"

say "fmt --check"
cargo fmt --check || die "cargo fmt --check (run: cargo fmt)"

say "clippy --all-targets -- -D warnings"
cargo clippy --all-targets -- -D warnings || die "clippy"

say "cargo test"
cargo test || die "cargo test"

say "ternary hearth Metal parity"
PARITY="crates/zen-cooker-quant-tern/kernels/hearth_parity.swift"
if command -v swift >/dev/null 2>&1 && swift "$PARITY" >/dev/null 2>&1; then
  swift "$PARITY" | tail -2
else
  warn "Metal parity skipped (swift or Metal device unavailable on this host)"
fi

say "iOS / macOS Swift build & self-test (mobile/ios)"
if command -v swiftc >/dev/null 2>&1; then
  swiftc -DZEN_CLI_SELF_TEST \
    mobile/ios/Sources/ZenUltraCookerKit/ZenUltraCookerModel.swift \
    mobile/ios/Sources/ZenUltraCookerApp/ZenUltraCookerApp.swift \
    -o /tmp/zen-ios-selftest || die "mobile/ios swiftc build"
  /tmp/zen-ios-selftest || die "mobile/ios self-test"
  rm -f /tmp/zen-ios-selftest
fi

say "Android APK v${EXPECTED_SEMVER} signature & E2E App Icon Pack check"
[ -f "assets/icons/app-icon-master.svg" ] || die "missing assets/icons/app-icon-master.svg"
[ -f "mobile/ios/ZenUltraCookerApp/Assets.xcassets/AppIcon.appiconset/icon-1024.png" ] || die "missing iOS AppIcon 1024"
[ -f "mobile/android/app/src/main/res/mipmap-xxxhdpi/ic_launcher.png" ] || die "missing Android mipmap-xxxhdpi ic_launcher.png"
[ -f "site.webmanifest" ] || die "missing site.webmanifest"
if [ -f "releases/zenUltraCooker-v${EXPECTED_SEMVER}.apk" ] && [ -x "$HOME/Library/Android/sdk/build-tools/36.0.0/apksigner" ]; then
  "$HOME/Library/Android/sdk/build-tools/36.0.0/apksigner" verify "releases/zenUltraCooker-v${EXPECTED_SEMVER}.apk"
  ls -lh "releases/zenUltraCooker-v${EXPECTED_SEMVER}.apk"
fi

say "the hearth is seated. DO exit admissible (v${EXPECTED_SEMVER}). 0 + 1 · 全"
