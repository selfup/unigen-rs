use super::quark:: {Quark, QuarkData};

#[derive(Debug, Copy, Clone)]
pub struct Protons {
    pub count: u32,
    pub protons: [ProtonData; 118],
}

impl Protons {
    pub fn new(count: u32) -> Self {
        let mut protons = [ProtonData::Unknown; 118];

        for idx in 0..count as usize {
            let proton = Proton::new();

            let proton_data = ProtonData::new(proton);

            protons[idx] = proton_data;
        }
        
        Self {
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
