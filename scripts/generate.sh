#!/usr/bin/env bash

set -eo pipefail


cargo build -q --release

if [[ $2 == 'warmup' ]]
then
    echo '
---
Warming up!
---
    '

    time cargo run -q --release 30
    
    echo '
---
Warmup done!
Running with given size now.
---
    '
fi

if [[ $1 == '' ]]
then
    echo 'Please pass a number to the script like so: ./scripts/generate.sh 30'
    echo '$1 arg not given - no size for generator..'
    echo 'Aborting!'
    exit 1
fi

time cargo run -q --release $1

cd ../../

