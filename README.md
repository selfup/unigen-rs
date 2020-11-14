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
Snapshot..

Block { id: 0, x: 0, y: 0, z: 0, charge: 0, atom: Atom { electrons: 20, nucleus: Nucleus { protons: 62, neutrons: 26 } } }

Universe built!
Checking the charge..
Field is Anionic
Size of Universe: 27000
```

![image](https://user-images.githubusercontent.com/9837366/99133467-5dfd2680-25d7-11eb-9377-de7feb36336d.png)
