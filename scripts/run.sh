#!/usr/bin/env bash

set -euox pipefail

cargo build

cargo run $1
