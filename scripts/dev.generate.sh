#!/usr/bin/env bash

set -eo pipefail

cargo build --quiet

time cargo run --quiet -- $1
