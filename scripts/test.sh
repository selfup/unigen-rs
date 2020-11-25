#!/usr/bin/env bash

set -euo pipefail

# simulator
cargo test --release -- --nocapture

# unigen
cd crates/unigen

cargo test --release -- --nocapture

cd ../..
