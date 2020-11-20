#!/usr/bin/env bash

set -euo pipefail

cd crates/unigen

cargo build -q --release

time cargo run -q --release $1

cd ../../
