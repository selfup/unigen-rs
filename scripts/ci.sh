#!/usr/bin/env bash

set -eo pipefail

echo "
$(date)"

./scripts/test.sh
