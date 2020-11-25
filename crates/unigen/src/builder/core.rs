pub mod quark;

use quark:: {Quark, QuarkData, Quarks};

#[derive(Debug, Copy, Clone)]
pub struct Atom {
    pub electrons: u32,
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
pub struct Block {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub charge: i8,
    pub atom: Atom,
}

#[derive(Debug, Copy, Clone)]
pub struct Protons {
    pub count: u32,
    pub protons: [ProtonData; 118],
}

impl Protons {
    pub fn new(count: u32) -> Protons {
        let mut protons = [ProtonData::Unknown; 118];

        for idx in 0..count as usize {
            let proton = Proton::new();

            let proton_data = ProtonData::new(proton);

            protons[idx] = proton_data;
        }
        
        Protons {
            count, 
            protons,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Proton {
    // Protons need up, up, down quarks
    pub quarks: (Quark, Quark, Quark),
}

impl Proton {
    pub fn new() -> Self {
         Self {
            quarks: (
                Quark::new(0, 0),
                Quark::new(1, 1),
                Quark::new(2, 1),
            )
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Neutrons {
    pub count: u32,
    pub neutrons: [NeutronData; 118],
}

impl Neutrons {
    pub fn new(count: u32) -> Neutrons {
        let mut neutrons = [NeutronData::Unknown; 118];

        for idx in 0..count as usize {
            let neutron = Neutron::new();

            let neutron_data = NeutronData::new(neutron);

            neutrons[idx] = neutron_data;
        }
        
        Neutrons {
            count, 
            neutrons,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Neutron {
    // Neutrons need up, down, down quarks
    pub quarks: (Quark, Quark, Quark),
}

impl Neutron {
    pub fn new() -> Neutron {
        Neutron {
            quarks: (
                Quark::new(0, 1),
                Quark::new(1, 0),
                Quark::new(2, 0),
            )
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ProtonData {
    Unknown,
    RedUpUpDownQuark,
    BlueUpUpDownQuark,
    GreenUpUpDownQuark,
    AlphaUpUpDownQuark,
}

impl ProtonData {
    pub fn new(proton: Proton) -> Self {
        let first_quark: QuarkData = Quark::data(proton.quarks.0);
        let second_quark: QuarkData = Quark::data(proton.quarks.1);
        let third_quark: QuarkData = Quark::data(proton.quarks.2);

        match (first_quark, second_quark, third_quark) {
            (QuarkData::RedUpQuark, QuarkData::RedUpQuark, QuarkData::RedDownQuark) =>
                ProtonData::RedUpUpDownQuark,
            
            (QuarkData::BlueUpQuark, QuarkData::BlueUpQuark, QuarkData::BlueDownQuark) =>
                ProtonData::BlueUpUpDownQuark,
            
            (QuarkData::GreenUpQuark, QuarkData::GreenUpQuark, QuarkData::GreenDownQuark) =>
                ProtonData::GreenUpUpDownQuark,
            
            (QuarkData::AlphaUpQuark, QuarkData::AlphaUpQuark, QuarkData::AlphaDownQuark) =>
                ProtonData::AlphaUpUpDownQuark,
            
            _ => ProtonData::Unknown,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum NeutronData {
    Unknown,
    RedUpDownDownQuark,
    BlueUpDownDownQuark,
    GreenUpDownDownQuark,
    AlphaUpDownDownQuark,
}

impl NeutronData {
    pub fn new(neutron: Neutron) -> Self {
        let first_quark: QuarkData = Quark::data(neutron.quarks.0);
        let second_quark: QuarkData = Quark::data(neutron.quarks.1);
        let third_quark: QuarkData = Quark::data(neutron.quarks.2);

        match (first_quark, second_quark, third_quark) {
            (QuarkData::RedUpQuark, QuarkData::RedDownQuark, QuarkData::RedDownQuark) =>
                NeutronData::RedUpDownDownQuark,
            
            (QuarkData::BlueUpQuark, QuarkData::BlueDownQuark, QuarkData::BlueDownQuark) =>
                NeutronData::BlueUpDownDownQuark,
            
            (QuarkData::GreenUpQuark, QuarkData::GreenDownQuark, QuarkData::GreenDownQuark) =>
                NeutronData::GreenUpDownDownQuark,
            
            (QuarkData::AlphaUpQuark, QuarkData::AlphaDownQuark, QuarkData::AlphaDownQuark) =>
                NeutronData::AlphaUpDownDownQuark,
            
            _ => NeutronData::Unknown,
        }
    }
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
