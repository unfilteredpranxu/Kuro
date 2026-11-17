#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
. scripts/env.sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
dx build --platform web --package kuro-web --release --locked
