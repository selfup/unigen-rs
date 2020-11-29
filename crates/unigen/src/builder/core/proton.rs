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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ProtonData {
    Unknown,

    RedBlueGreenUpUpDownQuark,
    RedBlueAlphaUpUpDownQuark,
    RedGreenBlueUpUpDownQuark,
    RedGreenAlphaUpUpDownQuark,
    RedAlphaBlueUpUpDownQuark,
    RedAlphaGreenUpUpDownQuark,

    GreenBlueRedUpUpDownQuark,
    GreenBlueAlphaUpUpDownQuark,
    GreenRedBlueUpUpDownQuark,
    GreenRedAlphaUpUpDownQuark,
    GreenAlphaBlueUpUpDownQuark,
    GreenAlphaRedUpUpDownQuark,

    BlueGreenRedUpUpDownQuark,
    BlueGreenAlphaUpUpDownQuark,
    BlueRedGreenUpUpDownQuark,
    BlueRedAlphaUpUpDownQuark,
    BlueAlphaGreenUpUpDownQuark,
    BlueAlphaRedUpUpDownQuark,

    AlphaBlueRedUpUpDownQuark,
    AlphaBlueGreenUpUpDownQuark,
    AlphaRedBlueUpUpDownQuark,
    AlphaRedGreenUpUpDownQuark,
    AlphaGreenRedUpUpDownQuark,
    AlphaGeenBlueUpUpDownQuark,
}

impl ProtonData {
    pub fn new(proton: Proton) -> Self {
        let first_quark: QuarkData = Quark::data(proton.quarks.0);
        let second_quark: QuarkData = Quark::data(proton.quarks.1);
        let third_quark: QuarkData = Quark::data(proton.quarks.2);

        match (first_quark, second_quark, third_quark) {
            (QuarkData::RedUpQuark, QuarkData::BlueUpQuark, QuarkData::GreenDownQuark) =>
                ProtonData::RedBlueGreenUpUpDownQuark,

            (QuarkData::RedUpQuark, QuarkData::BlueUpQuark, QuarkData::AlphaDownQuark) =>
                ProtonData::RedBlueAlphaUpUpDownQuark,

            (QuarkData::RedUpQuark, QuarkData::GreenUpQuark, QuarkData::BlueDownQuark) =>
                ProtonData::RedBlueGreenUpUpDownQuark,

            (QuarkData::RedUpQuark, QuarkData::GreenUpQuark, QuarkData::AlphaDownQuark) =>
                ProtonData::RedBlueGreenUpUpDownQuark,

            (QuarkData::RedUpQuark, QuarkData::AlphaUpQuark, QuarkData::BlueDownQuark) =>
                ProtonData::RedBlueGreenUpUpDownQuark,

            (QuarkData::RedUpQuark, QuarkData::AlphaUpQuark, QuarkData::GreenDownQuark) =>
                ProtonData::RedBlueGreenUpUpDownQuark,

            _ => ProtonData::Unknown,
        }
    }
}

#[test]
fn it_defaults_all_protons_as_unknown() {
    let protons = Protons::new(42);

    for proton in protons.protons.iter() {
        assert_eq!(proton, &ProtonData::Unknown);
    }
}
