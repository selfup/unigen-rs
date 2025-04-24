![github-pipeline](https://github.com/selfup/unigen-rs/actions/workflows/rust.yml/badge.svg)
![gitlab-pipeline](https://gitlab.com/selfup/unigen-rs/badges/main/pipeline.svg)

# Unigen

The core universe generator for [oxidizy](https://github.com/selfup/oxidizy).

This is a _reduced-scope_ **fork** of `oxidizy`: https://github.com/selfup/oxidizy

The history is saved but the scope of this project is down to the `unigen` crate.

This was done for performance tuning and feature growth. Oxidizy will rely on this crate moving forward.

# How to use from source

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
