#!/usr/bin/env bash
# build-apk.sh — Deterministic Android APK builder for 8b-is/zenUltraCooker (SemVer v2.1.0 / versionCode 20100)
#
# Uses the installed Android SDK (aapt2, d8, zipalign, apksigner) + javac + keytool
# to compile, package, align, sign, and verify `releases/zenUltraCooker-v<VERSION>.apk`.

set -euo pipefail
cd "$(dirname "$0")/.."

VERSION="$(tr -d '[:space:]' < VERSION)"
SDK_ROOT="${ANDROID_HOME:-$HOME/Library/Android/sdk}"
BUILD_TOOLS="$SDK_ROOT/build-tools/36.0.0"
PLATFORM_JAR="$SDK_ROOT/platforms/android-36/android.jar"

if [ ! -x "$BUILD_TOOLS/aapt2" ]; then
  BUILD_TOOLS="$SDK_ROOT/build-tools/34.0.0"
  PLATFORM_JAR="$SDK_ROOT/platforms/android-34/android.jar"
fi

AAPT2="$BUILD_TOOLS/aapt2"
D8="$BUILD_TOOLS/d8"
ZIPALIGN="$BUILD_TOOLS/zipalign"
APKSIGNER="$BUILD_TOOLS/apksigner"

ANDROID_DIR="mobile/android/app/src/main"
BUILD_DIR="target/android-build"
RELEASE_DIR="releases"
OUT_APK="$RELEASE_DIR/zenUltraCooker-v${VERSION}.apk"

rm -rf "$BUILD_DIR" "$ANDROID_DIR/assets/www/assets"
mkdir -p "$BUILD_DIR/compiled_res" "$BUILD_DIR/classes" "$BUILD_DIR/dex" "$ANDROID_DIR/assets/www/assets/icons" "$RELEASE_DIR"

echo "[apk-build] Syncing web app (v${VERSION}) into Android assets..."
cp index.html "$ANDROID_DIR/assets/www/index.html"
cp assets/brand-hero.svg "$ANDROID_DIR/assets/www/assets/brand-hero.svg"
cp assets/icons/app-icon-master.svg "$ANDROID_DIR/assets/www/assets/icons/app-icon-master.svg"
cp assets/icons/icon-192.png "$ANDROID_DIR/assets/www/assets/icons/icon-192.png"

echo "[apk-build] Compiling Android resources with aapt2..."
"$AAPT2" compile --dir "$ANDROID_DIR/res" -o "$BUILD_DIR/compiled_res/resources.zip"

echo "[apk-build] Linking base APK & generating R.java..."
"$AAPT2" link \
  -o "$BUILD_DIR/unaligned.apk" \
  -I "$PLATFORM_JAR" \
  --manifest "$ANDROID_DIR/AndroidManifest.xml" \
  --java "$BUILD_DIR/gen" \
  -A "$ANDROID_DIR/assets" \
  --auto-add-overlay \
  "$BUILD_DIR/compiled_res/resources.zip"

echo "[apk-build] Compiling Java sources (MainActivity + R.java)..."
javac --release 11 \
  -cp "$PLATFORM_JAR" \
  -d "$BUILD_DIR/classes" \
  "$ANDROID_DIR/java/is/eightb/zenultracooker/MainActivity.java" \
  "$BUILD_DIR/gen/is/eightb/zenultracooker/R.java"

echo "[apk-build] Dexing bytecode with d8..."
CLASS_FILES=()
while IFS= read -r -d '' f; do
  CLASS_FILES+=("$f")
done < <(find "$BUILD_DIR/classes" -name "*.class" -print0)

"$D8" --lib "$PLATFORM_JAR" --output "$BUILD_DIR/dex" "${CLASS_FILES[@]}"

echo "[apk-build] Adding classes.dex to APK..."
(cd "$BUILD_DIR/dex" && zip -q ../unaligned.apk classes.dex)

echo "[apk-build] Aligning APK on 4-byte boundary with zipalign..."
"$ZIPALIGN" -f -p 4 "$BUILD_DIR/unaligned.apk" "$BUILD_DIR/aligned.apk"

KEYSTORE="$BUILD_DIR/pawshaw-debug.keystore"
keytool -genkeypair \
  -keystore "$KEYSTORE" \
  -storepass android \
  -keypass android \
  -alias pawshaw \
  -keyalg RSA \
  -keysize 2048 \
  -validity 10000 \
  -dname "CN=8b-is zenUltraCooker, OU=PawShaw Hearth, O=8b.is, L=Tokyo, ST=Tokyo, C=JP" >/dev/null 2>&1

echo "[apk-build] Signing APK (v1 + v2 + v3 schemes) with apksigner..."
"$APKSIGNER" sign \
  --ks "$KEYSTORE" \
  --ks-pass pass:android \
  --key-pass pass:android \
  --out "$OUT_APK" \
  "$BUILD_DIR/aligned.apk"

echo "[apk-build] Verifying APK signature..."
"$APKSIGNER" verify --verbose "$OUT_APK" | head -n 6
ls -lh "$OUT_APK"
echo "[apk-build] SUCCESS -> $OUT_APK"
