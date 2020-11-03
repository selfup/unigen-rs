#!/usr/bin/env bash

set -euox pipefail

cargo build -q --release

time cargo run -q --release $1
