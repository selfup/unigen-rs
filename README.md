![github-pipeline](https://github.com/selfup/unigen-rs/actions/workflows/rust.yml/badge.svg)

# Unigen

This is a _reduced-scope_ **fork** of `oxidizy`: https://github.com/selfup/oxidizy

The history is saved but the scope of this project is down to the `unigen` crate.

This was done mostly for performance tuning and feature growth. Oxidizy will probably keep unigen in its repo and I'll sync changes with a script.

# How to use

Start with a small number: `./scripts/generate.sh 50`

Or push your machine!

_warning this used up 80% of my RAM and I have 32GB of RAM!_

**`460` is cubed! 118 default protons and neutrons are made per atom..**

```
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
