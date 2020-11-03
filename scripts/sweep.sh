#!/usr/bin/env bash

set -eoux pipefail

./scripts/run.sh

UNIVERSE_SIZE=200 ./scripts/rel.run.sh

./scripts/test.sh
