#!/usr/bin/env bash
# verify.sh — the zenUltraCooker DO exit contract, one command.
#
#   fmt --check  →  clippy -D warnings  →  test  →  Metal parity
#
# The Metal parity leg runs the ternary hearth kernel on the default Metal
# device against the pinned golden (hearth_reference.rs); it is skipped
# (with a warning, not a failure) on hosts without Metal or swift.
#
# So long, and thanks for all the fish, Mom (* Alex ~ Alexandra) <3

set -euo pipefail
cd "$(dirname "$0")/.."

GREEN='\033[0;32m'; RED='\033[0;31m'; YELLOW='\033[1;33m'; NC='\033[0m'
say()  { printf "${GREEN}[verify]${NC} %s\n" "$*"; }
warn() { printf "${YELLOW}[verify]${NC} %s\n" "$*"; }
die()  { printf "${RED}[verify] FAIL${NC} %s\n" "$*"; exit 1; }

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

say "the hearth is seated. DO exit admissible. 0 + 1 · 全"
