#[derive(Debug)]
pub struct Atom {
    pub electrons: i16,
    pub nucleus: Nucleus,
}

#[derive(Debug)]
pub struct Nucleus {
    pub protons: i16,
    pub neutrons: i16,
}

#[derive(Debug)]
pub struct LifeBlock {
    pub x_y: (u64, u64),
    pub z: u64,
    pub charge: i16,
    pub atom: Atom,
}
