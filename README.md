# Oxidizy

An ambitious project to mimic the smaller things in life.

This is how I learned rust.

The idea was to build a cli tool that can generate enough life, and _sublife_ (neutrons, protons, electrons), that when you give the tool atomic weights, it can "generate" appropriate elements.

Now it uses Bevy and it's amazing ECS system to constantly iterate and update atoms on the fly with new particles.

Another ambitious goal, would be to set up bonding logic to create water or other compounds.

The most difficult part will be setting up an electro-magnetic field. That is why we are starting with the simplest form of an electro magnetic field.

More to come :rocket:

---

`cargo run --release <n>`

Example Use:

```bash
$ ./scripts/rel.run.sh 30
+ cargo build -q --release
+ cargo run -q --release 30
Building Universe..
Threads: 16
Universe built!
Checking the charge..
Field is Anionic
Amount of Atoms in Universe: 27000
```

![image](https://user-images.githubusercontent.com/9837366/99208853-dce09380-277e-11eb-88be-e07d2044b10c.png)

### Sans UI

Bevy/ECS code is not tested. Still in beta and too many refactorings will take place over the next year or so.

However if you want to fill up a bunch of RAM and see how performant the `builder::generate_universe` is, you can run the generate script:

_warning this used up 80% of my RAM and I have 32GB of RAM!_

**`190` is cubed and then 118 default protons and neutrons are made per atom!**

```
$ ./scripts/generate.sh 190
+ cd crates/unigen
+ cargo build --release
    Finished release [optimized] target(s) in 0.09s
+ cargo run --release 190
    Finished release [optimized] target(s) in 0.09s
     Running `C:\Users\boudi\Documents\Rust\oxidizy\target\release\unigen.exe 190`
190
Building Universe..
Threads: 16
Universe built!
Checking the charge..
Field is Cationic
Amount of Atoms in Universe: 6859000
Amount of protons and neutrons: 1618724000

real    0m4.562s
user    0m0.000s
sys     0m0.015s
+ cd -
```
