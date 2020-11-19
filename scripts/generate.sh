#!/usr/bin/env bash

set -euox pipefail

cd crates/unigen

cargo build --release

time cargo run --release $1

cd -
