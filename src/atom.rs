#[derive(Debug)]
pub struct Nucleus {
    pub protons:  i32,
    pub neutrons: i32,
}

#[derive(Debug)]
pub struct Atom {
    pub electrons: i32,
    pub nucleus: Nucleus,
}
