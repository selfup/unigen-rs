#!/usr/bin/env bash

set -euo pipefail

# build for CI
cargo build --verbose

# unigen generate
cargo test -- --nocapture
