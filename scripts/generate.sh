#!/usr/bin/env bash

set -euo pipefail


( cd crates/unigen && time cargo run -q --release "$1")
