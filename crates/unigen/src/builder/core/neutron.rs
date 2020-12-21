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
    AlphaGreenBlueUpDownDownQuark,
}

impl NeutronData {
    pub fn new(neutron: Neutron) -> Self {
        let first_quark: QuarkData = Quark::data(neutron.quarks.0);
        let second_quark: QuarkData = Quark::data(neutron.quarks.1);
        let third_quark: QuarkData = Quark::data(neutron.quarks.2);

        match first_quark {
            QuarkData::RedUpQuark => {
                match (first_quark, second_quark, third_quark) {
                    (QuarkData::RedUpQuark, QuarkData::BlueDownQuark, QuarkData::GreenDownQuark) =>
                        NeutronData::RedBlueGreenUpDownDownQuark,

                    (QuarkData::RedUpQuark, QuarkData::BlueDownQuark, QuarkData::AlphaDownQuark) =>
                        NeutronData::RedBlueAlphaUpDownDownQuark,

                    (QuarkData::RedUpQuark, QuarkData::GreenDownQuark, QuarkData::BlueDownQuark) =>
                        NeutronData::RedGreenBlueUpDownDownQuark,

                    (QuarkData::RedUpQuark, QuarkData::GreenDownQuark, QuarkData::AlphaDownQuark) =>
                        NeutronData::RedGreenAlphaUpDownDownQuark,

                    (QuarkData::RedUpQuark, QuarkData::AlphaDownQuark, QuarkData::BlueDownQuark) =>
                        NeutronData::RedAlphaGreenUpDownDownQuark,

                    (QuarkData::RedUpQuark, QuarkData::AlphaDownQuark, QuarkData::GreenDownQuark) =>
                        NeutronData::RedAlphaGreenUpDownDownQuark,

                    _ => NeutronData::Unknown,
                }
            },

            QuarkData::GreenUpQuark => {
                match (first_quark, second_quark, third_quark) {
                    (QuarkData::GreenUpQuark, QuarkData::BlueDownQuark, QuarkData::RedDownQuark) =>
                        NeutronData::GreenBlueRedUpDownDownQuark,
    
                    (QuarkData::GreenUpQuark, QuarkData::BlueDownQuark, QuarkData::AlphaDownQuark) =>
                        NeutronData::GreenBlueAlphaUpDownDownQuark,
        
                    (QuarkData::GreenUpQuark, QuarkData::RedDownQuark, QuarkData::BlueDownQuark) =>
                        NeutronData::GreenRedBlueUpDownDownQuark,
        
                    (QuarkData::GreenUpQuark, QuarkData::RedDownQuark, QuarkData::AlphaDownQuark) =>
                        NeutronData::GreenRedAlphaUpDownDownQuark,
        
                    (QuarkData::GreenUpQuark, QuarkData::AlphaDownQuark, QuarkData::BlueDownQuark) =>
                        NeutronData::GreenAlphaBlueUpDownDownQuark,
        
                    (QuarkData::GreenUpQuark, QuarkData::AlphaDownQuark, QuarkData::RedDownQuark) =>
                        NeutronData::GreenAlphaRedUpDownDownQuark,

                    _ => NeutronData::Unknown,
                }
            },


            QuarkData::BlueUpQuark => {
                match (first_quark, second_quark, third_quark) {
                    (QuarkData::BlueUpQuark, QuarkData::GreenDownQuark, QuarkData::RedDownQuark) =>
                        NeutronData::BlueGreenRedUpDownDownQuark,
        
                    (QuarkData::BlueUpQuark, QuarkData::GreenDownQuark, QuarkData::AlphaDownQuark) =>
                        NeutronData::BlueGreenAlphaUpDownDownQuark,
        
                    (QuarkData::BlueUpQuark, QuarkData::RedDownQuark, QuarkData::GreenDownQuark) =>
                        NeutronData::BlueRedGreenUpDownDownQuark,
        
                    (QuarkData::BlueUpQuark, QuarkData::RedDownQuark, QuarkData::AlphaDownQuark) =>
                        NeutronData::BlueRedAlphaUpDownDownQuark,
        
                    (QuarkData::BlueUpQuark, QuarkData::AlphaDownQuark, QuarkData::GreenDownQuark) =>
                        NeutronData::BlueAlphaGreenUpDownDownQuark,
        
                    (QuarkData::BlueUpQuark, QuarkData::AlphaDownQuark, QuarkData::RedDownQuark) =>
                        NeutronData::BlueAlphaRedUpDownDownQuark,

                    _ => NeutronData::Unknown,
                }
            },

            QuarkData::AlphaUpQuark => {
                match (first_quark, second_quark, third_quark) {
                    (QuarkData::AlphaUpQuark, QuarkData::BlueDownQuark, QuarkData::RedDownQuark) =>
                        NeutronData::AlphaBlueRedUpDownDownQuark,
        
                    (QuarkData::AlphaUpQuark, QuarkData::BlueDownQuark, QuarkData::GreenDownQuark) =>
                        NeutronData::AlphaBlueGreenUpDownDownQuark,
        
                    (QuarkData::AlphaUpQuark, QuarkData::RedDownQuark, QuarkData::BlueDownQuark) =>
                        NeutronData::AlphaRedBlueUpDownDownQuark,
        
                    (QuarkData::AlphaUpQuark, QuarkData::RedDownQuark, QuarkData::GreenDownQuark) =>
                        NeutronData::AlphaRedGreenUpDownDownQuark,
        
                    (QuarkData::AlphaUpQuark, QuarkData::GreenDownQuark, QuarkData::RedDownQuark) =>
                        NeutronData::AlphaGreenRedUpDownDownQuark,
        
                    (QuarkData::AlphaUpQuark, QuarkData::GreenDownQuark, QuarkData::BlueDownQuark) =>
                        NeutronData::AlphaGreenBlueUpDownDownQuark,   

                    _ => NeutronData::Unknown,
                }
            },

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

    assert_eq!(first_neutron, NeutronData::RedGreenBlueUpDownDownQuark);
}
