![github-pipeline](https://github.com/selfup/unigen-rs/actions/workflows/rust.yml/badge.svg)
![gitlab-pipeline](https://gitlab.com/selfup/unigen-rs/badges/main/pipeline.svg)

# Unigen

The core universe generator for [oxidizy](https://github.com/selfup/oxidizy).

This is a _reduced-scope_ **fork** of `oxidizy`: https://github.com/selfup/oxidizy

The history is saved but the scope of this project is down to the `unigen` crate.

This was done for performance tuning and feature growth. Oxidizy will rely on this crate moving forward.

# How to use from source

## Benchmarks

Run `cargo bench --locked --bench universe` to measure initialization, tick,
particle totals, atom charge, and the complete compute pipeline without console
output. Results include time and blocks per second. Default side lengths are
5, 10, and 20 (125, 1,000, and 8,000 blocks).

The suite prints the worker count and actual block storage size. It runs sizes
sequentially and retains at most a fixture and one working universe, keeping
block storage to `2 * 8,000 * size_of::<Block>()` bytes by default, plus allocator,
Rayon, and benchmark harness overhead. Mutating phase benchmarks clone their
fixture outside timing, one iteration at a time; initialization and pipeline
timings include allocation and deallocation. Particle totals reuse a populated
fixture. Fixtures are deterministic, but tick uses the production random generator.
Rayon workers are warmed before measurement.

```sh
# Compile and execute each benchmark once (also run by GitHub/GitLab CI).
cargo bench --locked --bench universe -- --test

# Save a baseline, then compare after making changes on the same machine.
cargo bench --locked --bench universe -- --save-baseline before
cargo bench --locked --bench universe -- --baseline before

# Compare a single worker with the default pool in separate processes.
RAYON_NUM_THREADS=1 cargo bench --locked --bench universe

# Opt into side length 50 locally, in addition to the default sizes.
UNIGEN_BENCH_LARGE=1 cargo bench --locked --bench universe
```

In PowerShell, set `$env:RAYON_NUM_THREADS = "1"` or
`$env:UNIGEN_BENCH_LARGE = "1"` before running Cargo; remove the variable afterward
with `Remove-Item Env:RAYON_NUM_THREADS` or `Remove-Item Env:UNIGEN_BENCH_LARGE`.
Benchmark names include worker count. Results are stored under `target/criterion`.
Record CPU, Rust version (`rustc -Vv`), and worker count with comparisons. Small
workloads emphasize scheduling overhead; these results do not predict large-world
throughput. CI runs smoke tests without timing thresholds.

## Generate a universe

Start with a small number: 

* Shell: `./scripts/generate.sh 50`
* Powershell: _scripts available in the_ `pwsh` _dir_

Example output:

```console
./scripts/generate.sh 50
--------------------------------
Threads: 10
Building..
--------------------------------
Universe built
--------------------------------
Calculating charge of field..
--------------------------------
Field is Cationic
--------------------------------
Atoms: 125,000
Baryons: 29,500,000
Quarks: 88,500,000
--------------------------------
Total high-level objects in memory: 118,125,000
--------------------------------

real    0m0.203s
user    0m0.035s
sys     0m0.045s
```

### Or push your machine!

_warning this used up 80% of my RAM with a machine that has 32GB of DDR4 RAM (3200Mhz)!_

**`460` is cubed! 118 default protons and neutrons are made per atom..**

```console
$ ./scripts/generate.sh 460
--------------------------------
Threads: 16
Building..
--------------------------------
Universe built
--------------------------------
Calculating charge of field..
--------------------------------
Field is Anionic
--------------------------------
Atoms: 97336000
Baryons: 22971296000
Quarks: 68913888000
--------------------------------
Total objects in memory: 91982520000
--------------------------------

real    0m6.070s
user    0m0.000s
sys     0m0.015s
```
