#!/usr/bin/env bash

set -eo pipefail

ROOT_SIZE=0

if [[ $1 -eq '' ]]
then
    ROOT_SIZE=200
else
    ROOT_SIZE=$1
fi

echo "
$(date)"

./scripts/test.sh

echo '
---
'

./scripts/rel.run.sh $ROOT_SIZE

echo "
$(date)"
