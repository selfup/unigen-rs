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
$ ./scripts/rel.run.sh 30
+ cargo build -q --release
+ cargo run -q --release 30
Building Universe..
Threads: 16
Snapshot..

Block { id: 0, x: 0, y: 0, z: 0, charge: 0, atom: Atom { electrons: 20, nucleus: Nucleus { protons: 62, neutrons: 26 } } }

Universe built!
Checking the charge..
Field is Anionic
Size of Universe: 27000
```

TODO: Add screen shot of Bevy rendering the universe
