#!/usr/bin/env bash

set -euo pipefail

# simulator
cargo test -- --nocapture

# unigen
( cd crates/unigen && exec cargo test -- --nocapture )

