use defmt::Format;

#[derive(PartialEq, PartialOrd, Clone, Hash, Debug, Format)]
pub enum BusVoltageRange {
    UpTo16V = 0,
    UpTo32V = 1,
}

impl Default for BusVoltageRange {
    fn default() -> Self {
        Self::UpTo32V
    }
}
