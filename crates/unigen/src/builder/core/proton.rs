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
                Quark::new(0, 1),
                Quark::new(1, 1),
                Quark::new(2, 0),
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
    AlphaGreenBlueUpUpDownQuark,
}

impl ProtonData {
    pub fn new(proton: Proton) -> Self {
        let first_quark: QuarkData = Quark::data(proton.quarks.0);
        let second_quark: QuarkData = Quark::data(proton.quarks.1);
        let third_quark: QuarkData = Quark::data(proton.quarks.2);

        match (first_quark, second_quark, third_quark) {
            // RedUpQuark
            (QuarkData::RedUpQuark, QuarkData::BlueUpQuark, QuarkData::GreenDownQuark) =>
                ProtonData::RedBlueGreenUpUpDownQuark,

            (QuarkData::RedUpQuark, QuarkData::BlueUpQuark, QuarkData::AlphaDownQuark) =>
                ProtonData::RedBlueAlphaUpUpDownQuark,

            (QuarkData::RedUpQuark, QuarkData::GreenUpQuark, QuarkData::BlueDownQuark) =>
                ProtonData::RedGreenBlueUpUpDownQuark,

            (QuarkData::RedUpQuark, QuarkData::GreenUpQuark, QuarkData::AlphaDownQuark) =>
                ProtonData::RedGreenAlphaUpUpDownQuark,

            (QuarkData::RedUpQuark, QuarkData::AlphaUpQuark, QuarkData::BlueDownQuark) =>
                ProtonData::RedAlphaGreenUpUpDownQuark,

            (QuarkData::RedUpQuark, QuarkData::AlphaUpQuark, QuarkData::GreenDownQuark) =>
                ProtonData::RedAlphaGreenUpUpDownQuark,

            // GreenUpQuark
            (QuarkData::GreenUpQuark, QuarkData::BlueUpQuark, QuarkData::RedDownQuark) =>
                ProtonData::GreenBlueRedUpUpDownQuark,

            (QuarkData::GreenUpQuark, QuarkData::BlueUpQuark, QuarkData::AlphaDownQuark) =>
                ProtonData::GreenBlueAlphaUpUpDownQuark,

            (QuarkData::GreenUpQuark, QuarkData::RedUpQuark, QuarkData::BlueDownQuark) =>
                ProtonData::GreenRedBlueUpUpDownQuark,

            (QuarkData::GreenUpQuark, QuarkData::RedUpQuark, QuarkData::AlphaDownQuark) =>
                ProtonData::GreenRedAlphaUpUpDownQuark,

            (QuarkData::GreenUpQuark, QuarkData::AlphaUpQuark, QuarkData::BlueDownQuark) =>
                ProtonData::GreenAlphaBlueUpUpDownQuark,

            (QuarkData::GreenUpQuark, QuarkData::AlphaUpQuark, QuarkData::RedDownQuark) =>
                ProtonData::RedBlueGreenUpUpDownQuark,

            // BlueUpQuark
            (QuarkData::BlueUpQuark, QuarkData::GreenUpQuark, QuarkData::RedDownQuark) =>
                ProtonData::BlueGreenRedUpUpDownQuark,

            (QuarkData::BlueUpQuark, QuarkData::GreenUpQuark, QuarkData::AlphaDownQuark) =>
                ProtonData::BlueGreenAlphaUpUpDownQuark,

            (QuarkData::BlueUpQuark, QuarkData::RedUpQuark, QuarkData::GreenDownQuark) =>
                ProtonData::BlueRedGreenUpUpDownQuark,

            (QuarkData::BlueUpQuark, QuarkData::RedUpQuark, QuarkData::AlphaDownQuark) =>
                ProtonData::BlueRedAlphaUpUpDownQuark,

            (QuarkData::BlueUpQuark, QuarkData::AlphaUpQuark, QuarkData::GreenDownQuark) =>
                ProtonData::BlueAlphaGreenUpUpDownQuark,

            (QuarkData::BlueUpQuark, QuarkData::AlphaUpQuark, QuarkData::RedDownQuark) =>
                ProtonData::BlueAlphaRedUpUpDownQuark,
                
            // AlphaUpQuark
            (QuarkData::AlphaUpQuark, QuarkData::BlueUpQuark, QuarkData::RedDownQuark) =>
                ProtonData::AlphaBlueRedUpUpDownQuark,

            (QuarkData::AlphaUpQuark, QuarkData::BlueUpQuark, QuarkData::GreenDownQuark) =>
                ProtonData::AlphaBlueGreenUpUpDownQuark,

            (QuarkData::AlphaUpQuark, QuarkData::RedUpQuark, QuarkData::BlueDownQuark) =>
                ProtonData::AlphaRedBlueUpUpDownQuark,

            (QuarkData::AlphaUpQuark, QuarkData::RedUpQuark, QuarkData::GreenDownQuark) =>
                ProtonData::AlphaRedGreenUpUpDownQuark,

            (QuarkData::AlphaUpQuark, QuarkData::GreenUpQuark, QuarkData::RedDownQuark) =>
                ProtonData::AlphaGreenRedUpUpDownQuark,

            (QuarkData::AlphaUpQuark, QuarkData::GreenUpQuark, QuarkData::BlueDownQuark) =>
                ProtonData::AlphaGreenBlueUpUpDownQuark,         

            // Unknown
            _ => ProtonData::Unknown,
        }
    }
}

#[test]
fn it_defaults_all_protons_as_unknown() {
    let protons = Protons::new(0);

    for proton in protons.protons.iter() {
        assert_eq!(proton, &ProtonData::Unknown);
    }
}

#[test]
fn it_can_match_a_single_proton_correctly() {
    let protons = Protons::new(1);

    let first_proton = protons.protons[0];

    assert_eq!(first_proton, ProtonData::RedGreenBlueUpUpDownQuark);
}
