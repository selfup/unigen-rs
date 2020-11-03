#[derive(Debug, Copy, Clone)]
pub struct Atom {
    pub electrons: i16,
    pub nucleus: Nucleus,
}

#[derive(Debug, Copy, Clone)]
pub struct Nucleus {
    pub protons: i16,
    pub neutrons: i16,
}

#[derive(Debug, Copy, Clone)]
pub struct LifeBlock {
    pub x: u64,
    pub y: u64,
    pub z: u64,
    pub charge: i16,
    pub atom: Atom,
}

#[derive(Debug, Copy, Clone)]
pub struct Grid {
    pub x: u64,
    pub y: u64,
    pub z: u64,
    pub charge: i16,
    pub block: LifeBlock,
}
