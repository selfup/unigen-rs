#!/usr/bin/env bash

set -eo pipefail

cd crates/unigen

cargo build -q --release

if [[ $1 == '' ]]
then
    echo 'Please pass a number to the script like so: ./scripts/generate.sh 30'
    echo '$1 arg not given - no size for generator..'
    echo 'Aborting!'
    exit 1
fi

time cargo run -q --release $1

cd ../../
