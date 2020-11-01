#!/usr/bin/env bash

set -euox pipefail

UNIVERSE_SIZE=600

cargo build -q --release

time cargo run -q --release $UNIVERSE_SIZE
