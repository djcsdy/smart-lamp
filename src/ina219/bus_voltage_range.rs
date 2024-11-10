use defmt::Format;

#[derive(PartialEq, PartialOrd, Clone, Hash, Debug, Format)]
pub enum BusVoltageRange {
    UpTo16V,
    UpTo32V,
}

impl Default for BusVoltageRange {
    fn default() -> Self {
        Self::UpTo32V
    }
}
