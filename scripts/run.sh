#!/usr/bin/env bash

set -euox pipefail

cargo build

time cargo run 200
