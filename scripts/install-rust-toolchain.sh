#!/usr/bin/env bash
# install-rust-toolchain.sh: Install the Rust toolchain (cargo, rustc, clippy,
# rustfmt) via Homebrew, per this factory's install policy
# (prefer_homebrew: true, avoid_opaque_installers: true, see .hekton/project.yaml
# and ~/hekton/bootstrap/install-cli-tools.sh for the same pattern).
#
# Scope note: this script lives in borrow-native for now, not
# ~/hekton/bootstrap/, since this is the first Hekton project that actually
# needs a Rust toolchain. If a second project needs it, promote this file
# (unchanged, it has no borrow-native-specific paths) to
# ~/hekton/bootstrap/install-rust-toolchain.sh and call it from
# ~/hekton/bootstrap/run-all.sh, the same promotion path any other bootstrap
# script would take.
#
# Usage:
#   scripts/install-rust-toolchain.sh           # install + verify
#   scripts/install-rust-toolchain.sh --check   # verify only, exit 1 if missing
#
# Network: brew install only (no rustup, no curl | sh — see the install
# policy's avoid_opaque_installers note in .hekton/project.yaml).
set -euo pipefail

CHECK_ONLY=false
[[ "${1:-}" == "--check" ]] && CHECK_ONLY=true

REQUIRED_CMDS=(cargo rustc)

missing=()
for cmd in "${REQUIRED_CMDS[@]}"; do
  command -v "$cmd" >/dev/null 2>&1 || missing+=("$cmd")
done

if [[ "$CHECK_ONLY" == true ]]; then
  if [[ ${#missing[@]} -eq 0 ]]; then
    echo "OK: Rust toolchain present ($(rustc --version))"
    exit 0
  else
    echo "MISSING: ${missing[*]} — run scripts/install-rust-toolchain.sh" >&2
    exit 1
  fi
fi

if [[ ${#missing[@]} -eq 0 ]]; then
  echo "Rust toolchain already installed:"
else
  if ! command -v brew >/dev/null 2>&1; then
    echo "ERROR: Homebrew not found. Install it first: https://brew.sh" >&2
    exit 1
  fi
  echo "🦀 Installing Rust toolchain via Homebrew..."
  brew install rust
fi

echo ""
echo "Installed:"
echo "  - cargo:  $(cargo --version)"
echo "  - rustc:  $(rustc --version)"
echo "  - clippy: $(cargo clippy --version 2>&1 || echo 'NOT FOUND — brew install rust should include it; check brew doctor')"
echo "  - fmt:    $(cargo fmt --version 2>&1 || echo 'NOT FOUND')"
echo ""
echo "Verify Module 01's deterministic gate works: cargo test && cargo clippy -- -D warnings"
