#!/usr/bin/env bash

set -eo pipefail

cd crates/unigen

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

time cargo run -q --release $1

cd ../../
