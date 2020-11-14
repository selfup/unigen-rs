#!/usr/bin/env bash

set -euox pipefail

cargo build -q --release

cargo run -q --release $1
