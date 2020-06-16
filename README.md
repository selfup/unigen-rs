# Oxidizy

An ambitious project to mimic the smaller things in life.

This is how I learned rust.

The idea is to build a cli tool that can generate enough life, and _sublife_ (neutrons, protons, electrons), that when you give the tool atomic weights, it can "generate" appropriate elements.

Another ambitious goal, would be to set up bonding logic to create water or other compounds.

The most difficult part will be setting up an electro-magnetic field. That is why we are starting with the simplest form of an electro magnetic field.

---

`cargo run --release <n>`

Example Use:

```bash
oxidizy (master) $ time cargo run -q --release 200
field is anionic
Size of Universe: 8000000

real    0m0.317s
user    0m0.233s
sys     0m0.085s
```
