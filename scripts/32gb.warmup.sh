#!/usr/bin/env bash

set -eo pipefail

cd crates/unigen

cargo build -q --release

echo '
---
Warming up!
---
'

for size in 30 50 70 100 200 300 400 480
do
    sleep 1s
    time cargo run -q --release $size
done
    
echo '
---
32GB of RAM Warmup done!
---'

cd ../../
