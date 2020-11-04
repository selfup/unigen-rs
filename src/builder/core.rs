#[derive(Debug, Copy, Clone)]
pub struct Atom {
    pub electrons: u32,
    pub nucleus: Nucleus,
}

#[derive(Debug, Copy, Clone)]
pub struct Nucleus {
    pub protons: u32,
    pub neutrons: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Block {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub charge: i8,
    pub atom: Atom,
}
