pub mod neutron;
pub mod proton;
pub mod quark;

use neutron::Neutrons;
use proton::Protons;
use quark::Quarks;

#[derive(Debug, Copy, Clone)]
pub struct Block {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub charge: i8,
    pub atom: Atom,
}

#[derive(Debug, Copy, Clone)]
pub struct Atom {
    pub electrons: u8,
    pub nucleus: Nucleus,
}

#[derive(Debug, Copy, Clone)]
pub struct Nucleus {
    pub baryon: Baryon,
}

#[derive(Debug, Copy, Clone)]
pub struct Baryon {
    pub protons: Protons,
    pub neutrons: Neutrons,
}

#[derive(Debug, Copy, Clone)]
pub struct Fermion {
    pub quarks: Quarks,
    pub leptons: Leptons,
}

#[derive(Debug, Copy, Clone)]
pub struct Leptons {
    pub e: Lepton,
    pub ve: Lepton,
    pub u: Lepton,
    pub vu: Lepton,
    pub t: Lepton,
    pub vt: Lepton,
}

#[derive(Debug, Copy, Clone)]
pub struct Lepton {}

#[derive(Debug, Copy, Clone)]
pub struct Bozons {}

#[derive(Debug, Copy, Clone)]
pub struct Hadrons {}

#[derive(Debug, Copy, Clone)]
pub enum Flip {
    Unknown,
    Zero,
    One,
    NegativeOne,
}

#[derive(Debug, Copy, Clone)]
pub struct MinkowskiSpace {
    pub flips: [Flip; 16],
}

#[cfg(test)]
mod tests {
    use super::neutron::Neutrons;
    use super::proton::Protons;
    use super::quark::{Quark, Quarks};
    use super::*;

    #[test]
    fn test_block_creation() {
        let block = Block {
            id: 1,
            x: 2,
            y: 3,
            z: 4,
            charge: 5,
            atom: Atom {
                electrons: 6,
                nucleus: Nucleus {
                    baryon: Baryon {
                        protons: Protons::new(0),
                        neutrons: Neutrons::new(0),
                    },
                },
            },
        };

        assert_eq!(block.id, 1);
        assert_eq!(block.x, 2);
        assert_eq!(block.y, 3);
        assert_eq!(block.z, 4);
        assert_eq!(block.charge, 5);
        assert_eq!(block.atom.electrons, 6);
    }

    #[test]
    fn test_atom_creation() {
        let atom = Atom {
            electrons: 1,
            nucleus: Nucleus {
                baryon: Baryon {
                    protons: Protons::new(0),
                    neutrons: Neutrons::new(0),
                },
            },
        };

        assert_eq!(atom.electrons, 1);
    }

    #[test]
    fn test_nucleus_creation() {
        let _nucleus = Nucleus {
            baryon: Baryon {
                protons: Protons::new(0),
                neutrons: Neutrons::new(0),
            },
        };
    }

    #[test]
    fn test_baryon_creation() {
        let _baryon = Baryon {
            protons: Protons::new(0),
            neutrons: Neutrons::new(0),
        };
    }

    #[test]
    fn test_fermion_creation() {
        let _fermion = Fermion {
            quarks: Quarks {
                up: Quark::new(0, 1),
                down: Quark::new(0, 0),
                charm: Quark::new(0, 1),
                strange: Quark::new(0, 0),
                top: Quark::new(0, 1),
                bottom: Quark::new(0, 0),
            },
            leptons: Leptons {
                e: Lepton {},
                ve: Lepton {},
                u: Lepton {},
                vu: Lepton {},
                t: Lepton {},
                vt: Lepton {},
            },
        };
    }

    #[test]
    fn test_leptons_creation() {
        let _leptons = Leptons {
            e: Lepton {},
            ve: Lepton {},
            u: Lepton {},
            vu: Lepton {},
            t: Lepton {},
            vt: Lepton {},
        };
    }

    #[test]
    fn test_flip_enum() {
        assert_eq!(Flip::Unknown as i32, Flip::Unknown as i32);
        assert_eq!(Flip::Zero as i32, Flip::Zero as i32);
        assert_eq!(Flip::One as i32, Flip::One as i32);
        assert_eq!(Flip::NegativeOne as i32, Flip::NegativeOne as i32);
    }

    #[test]
    fn test_minkowski_space_creation() {
        let _space = MinkowskiSpace {
            flips: [
                Flip::Unknown,
                Flip::Zero,
                Flip::One,
                Flip::NegativeOne,
                Flip::Unknown,
                Flip::Zero,
                Flip::One,
                Flip::NegativeOne,
                Flip::Unknown,
                Flip::Zero,
                Flip::One,
                Flip::NegativeOne,
                Flip::Unknown,
                Flip::Zero,
                Flip::One,
                Flip::NegativeOne,
            ],
        };
    }
}
