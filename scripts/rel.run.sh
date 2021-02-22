#!/usr/bin/env bash

set -euox pipefail

cargo run -q --release "$1"
