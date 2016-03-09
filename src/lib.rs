#[derive(Debug)]
struct Nucleus {
    protons:  i32,
    neutrons: i32,
}

#[derive(Debug)]
struct Atom {
    electrons: i32,
    nucleus: Nucleus,
}
