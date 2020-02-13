# Oxidizy

An ambitious project to mimic the smaller things in life.

This is how I learned rust.

The idea is to build a cli tool that can generate enough life, and *sublife* (neutrons, protons, electrons), that when you give the tool atomic weights, it can "generate" appropriate elements.

Another ambitious goal, would be to set up bonding logic to create water or other compounds.

The most difficult part will be setting up an electro-magnetic field. That is why we are starting with the simplest form of an electro magnetic field.

***

`cargo run --release <n>`

Example Use:

```bash
oxidizy (master) $ time cargo run --release 200
    Finished release [optimized] target(s) in 0.02s
     Running `target/release/oxidizy 200`
field is anionic
Size of Universe: 8120601

real	0m0.436s
user	0m0.501s
sys	0m0.141s
```
