use super::quark:: {Quark, QuarkData};

#[derive(Debug, Copy, Clone)]
pub struct Neutrons {
    pub count: u32,
    pub neutrons: [NeutronData; 118],
}

impl Neutrons {
    pub fn new(count: u32) -> Self {
        let mut neutrons = [NeutronData::Unknown; 118];

        for idx in 0..count as usize {
            let neutron = Neutron::new();

            let neutron_data = NeutronData::new(neutron);

            neutrons[idx] = neutron_data;
        }
        
        Self {
            count, 
            neutrons,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Neutron {
    pub quarks: (Quark, Quark, Quark),
}

impl Neutron {
    pub fn new() -> Self {
        Self {
            quarks: (
                Quark::new(0, 1),
                Quark::new(1, 0),
                Quark::new(2, 0),
            )
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NeutronData {
    Unknown,

    RedBlueGreenUpDownDownQuark,
    RedBlueAlphaUpDownDownQuark,
    RedGreenBlueUpDownDownQuark,
    RedGreenAlphaUpDownDownQuark,
    RedAlphaBlueUpDownDownQuark,
    RedAlphaGreenUpDownDownQuark,

    GreenBlueRedUpDownDownQuark,
    GreenBlueAlphaUpDownDownQuark,
    GreenRedBlueUpDownDownQuark,
    GreenRedAlphaUpDownDownQuark,
    GreenAlphaBlueUpDownDownQuark,
    GreenAlphaRedUpDownDownQuark,

    BlueGreenRedUpDownDownQuark,
    BlueGreenAlphaUpDownDownQuark,
    BlueRedGreenUpDownDownQuark,
    BlueRedAlphaUpDownDownQuark,
    BlueAlphaGreenUpDownDownQuark,
    BlueAlphaRedUpDownDownQuark,

    AlphaBlueRedUpDownDownQuark,
    AlphaBlueGreenUpDownDownQuark,
    AlphaRedBlueUpDownDownQuark,
    AlphaRedGreenUpDownDownQuark,
    AlphaGreenRedUpDownDownQuark,
    AlphaGeenBlueUpDownDownQuark,
}

impl NeutronData {
    pub fn new(neutron: Neutron) -> Self {
        let first_quark: QuarkData = Quark::data(neutron.quarks.0);
        let second_quark: QuarkData = Quark::data(neutron.quarks.1);
        let third_quark: QuarkData = Quark::data(neutron.quarks.2);

        match (first_quark, second_quark, third_quark) {
            // RedUpQuark
            (QuarkData::RedUpQuark, QuarkData::BlueDownQuark, QuarkData::GreenDownQuark) =>
                NeutronData::RedBlueGreenUpDownDownQuark,

            (QuarkData::RedUpQuark, QuarkData::BlueDownQuark, QuarkData::AlphaDownQuark) =>
                NeutronData::RedBlueAlphaUpDownDownQuark,

            (QuarkData::RedUpQuark, QuarkData::GreenDownQuark, QuarkData::BlueDownQuark) =>
                NeutronData::RedBlueGreenUpDownDownQuark,

            (QuarkData::RedUpQuark, QuarkData::GreenDownQuark, QuarkData::AlphaDownQuark) =>
                NeutronData::RedBlueGreenUpDownDownQuark,

            (QuarkData::RedUpQuark, QuarkData::AlphaDownQuark, QuarkData::BlueDownQuark) =>
                NeutronData::RedBlueGreenUpDownDownQuark,

            (QuarkData::RedUpQuark, QuarkData::AlphaDownQuark, QuarkData::GreenDownQuark) =>
                NeutronData::RedBlueGreenUpDownDownQuark,

            // GreenUpQuark
            (QuarkData::GreenUpQuark, QuarkData::BlueDownQuark, QuarkData::RedDownQuark) =>
                NeutronData::RedBlueGreenUpDownDownQuark,

            (QuarkData::GreenUpQuark, QuarkData::BlueDownQuark, QuarkData::AlphaDownQuark) =>
                NeutronData::RedBlueAlphaUpDownDownQuark,

            (QuarkData::GreenUpQuark, QuarkData::RedDownQuark, QuarkData::BlueDownQuark) =>
                NeutronData::RedBlueGreenUpDownDownQuark,

            (QuarkData::GreenUpQuark, QuarkData::RedDownQuark, QuarkData::AlphaDownQuark) =>
                NeutronData::RedBlueGreenUpDownDownQuark,

            (QuarkData::GreenUpQuark, QuarkData::AlphaDownQuark, QuarkData::BlueDownQuark) =>
                NeutronData::RedBlueGreenUpDownDownQuark,

            (QuarkData::GreenUpQuark, QuarkData::AlphaDownQuark, QuarkData::RedDownQuark) =>
                NeutronData::RedBlueGreenUpDownDownQuark,

            // BlueUpQuark
            (QuarkData::BlueUpQuark, QuarkData::GreenDownQuark, QuarkData::RedDownQuark) =>
                NeutronData::RedBlueGreenUpDownDownQuark,

            (QuarkData::BlueUpQuark, QuarkData::GreenDownQuark, QuarkData::AlphaDownQuark) =>
                NeutronData::RedBlueAlphaUpDownDownQuark,

            (QuarkData::BlueUpQuark, QuarkData::RedDownQuark, QuarkData::GreenDownQuark) =>
                NeutronData::RedBlueGreenUpDownDownQuark,

            (QuarkData::BlueUpQuark, QuarkData::RedDownQuark, QuarkData::AlphaDownQuark) =>
                NeutronData::RedBlueGreenUpDownDownQuark,

            (QuarkData::BlueUpQuark, QuarkData::AlphaDownQuark, QuarkData::GreenDownQuark) =>
                NeutronData::RedBlueGreenUpDownDownQuark,

            (QuarkData::BlueUpQuark, QuarkData::AlphaDownQuark, QuarkData::RedDownQuark) =>
                NeutronData::RedBlueGreenUpDownDownQuark,
                
            // AlphaUpQuark
            (QuarkData::AlphaUpQuark, QuarkData::BlueDownQuark, QuarkData::RedDownQuark) =>
                NeutronData::RedBlueGreenUpDownDownQuark,

            (QuarkData::AlphaUpQuark, QuarkData::BlueDownQuark, QuarkData::GreenDownQuark) =>
                NeutronData::RedBlueAlphaUpDownDownQuark,

            (QuarkData::AlphaUpQuark, QuarkData::RedDownQuark, QuarkData::BlueDownQuark) =>
                NeutronData::RedBlueGreenUpDownDownQuark,

            (QuarkData::AlphaUpQuark, QuarkData::RedDownQuark, QuarkData::GreenDownQuark) =>
                NeutronData::RedBlueGreenUpDownDownQuark,

            (QuarkData::AlphaUpQuark, QuarkData::GreenDownQuark, QuarkData::RedDownQuark) =>
                NeutronData::RedBlueGreenUpDownDownQuark,

            (QuarkData::AlphaUpQuark, QuarkData::GreenDownQuark, QuarkData::BlueDownQuark) =>
                NeutronData::RedBlueGreenUpDownDownQuark,         

            // Unknown
            _ => NeutronData::Unknown,
        }
    }
}

#[test]
fn it_defaults_all_neutrons_as_unknown() {
    let neutrons = Neutrons::new(0);

    for neutron in neutrons.neutrons.iter() {
        assert_eq!(neutron, &NeutronData::Unknown);
    }
}

#[test]
fn it_can_match_a_single_neutron_correctly() {
    let neutrons = Neutrons::new(1);

    let first_neutron = neutrons.neutrons[0];

    assert_eq!(first_neutron, NeutronData::RedBlueGreenUpDownDownQuark);
}
