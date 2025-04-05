#!/usr/bin/env bash

set -eo pipefail

cargo build --release --quiet

time cargo run --release --quiet $1
