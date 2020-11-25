#[derive(Debug, Copy, Clone)]
pub struct Quarks {
    pub u: Quark,
    pub d: Quark,
    pub c: Quark,
    pub s: Quark,
    pub t: Quark,
    pub b: Quark,
}

#[derive(Debug, Copy, Clone)]
pub struct Quark {
    pub color: StrongCharge,
    pub elementary_charge: ElectricCharge,
}

impl Quark {
    pub fn new(color_charge: u8, electric_charge: u8) -> Quark {
        Quark{
            color: StrongCharge::new(color_charge),
            elementary_charge: ElectricCharge::new(electric_charge),
        }
    }

    pub fn data(quark: Quark) -> QuarkData {
        #[allow(unreachable_patterns)]
        match (quark.color, quark.elementary_charge) {
            (StrongCharge::Red, ElectricCharge::NegativeOneThird) => QuarkData::RedDownQuark,
            (StrongCharge::Red, ElectricCharge::PositiveTwoThirds) => QuarkData::RedUpQuark,

            (StrongCharge::Blue, ElectricCharge::NegativeOneThird) => QuarkData::BlueDownQuark, 
            (StrongCharge::Blue, ElectricCharge::PositiveTwoThirds) => QuarkData::BlueUpQuark, 
            
            (StrongCharge::Green, ElectricCharge::NegativeOneThird) => QuarkData::GreenDownQuark, 
            (StrongCharge::Green, ElectricCharge::PositiveTwoThirds) => QuarkData::GreenUpQuark, 
            
            (StrongCharge::Alpha, ElectricCharge::NegativeOneThird) => QuarkData::AlphaDownQuark, 
            (StrongCharge::Alpha, ElectricCharge::PositiveTwoThirds) => QuarkData::AlphaUpQuark,
            
            _ => QuarkData::Unknown,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum QuarkData {
    Unknown,
    RedUpQuark,
    RedDownQuark,
    BlueUpQuark,
    BlueDownQuark,
    GreenUpQuark,
    GreenDownQuark,
    AlphaUpQuark,
    AlphaDownQuark,
}

#[derive(Debug, Copy, Clone)]
pub enum StrongCharge {
    Red,
    Green,
    Blue,
    Alpha,
}

impl StrongCharge {
    pub fn new(color: u8) -> StrongCharge {
        match color {
            0 => StrongCharge::Red,
            1 => StrongCharge::Green,
            2 => StrongCharge::Blue,
            _ => StrongCharge::Alpha,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ElectricCharge {
    PositiveTwoThirds,
    NegativeOneThird,
}

impl ElectricCharge {
    pub fn new(charge: u8) -> ElectricCharge {
        match charge {
            0 => ElectricCharge::NegativeOneThird,
            _ => ElectricCharge::PositiveTwoThirds,
        }
    }
}
