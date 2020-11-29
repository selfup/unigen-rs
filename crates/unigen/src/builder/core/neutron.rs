use super::quark:: {Quark, QuarkData};

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
