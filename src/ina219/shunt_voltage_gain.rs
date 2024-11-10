use defmt::Format;

#[derive(PartialEq, PartialOrd, Clone, Hash, Debug, Format)]
pub enum ShuntVoltageGain {
    /// ×1 shunt voltage gain.
    ///
    /// Shunt voltage range is ±40mV.
    None = 0,
    /// ÷2 shunt voltage gain.
    ///
    /// Shunt voltage range is ±80mV.
    Half = 1,
    /// ÷4 shunt voltage gain.
    ///
    /// Shunt voltage range is ±160mV.
    Quarter = 2,
    /// ÷8 shunt voltage gain.
    ///
    /// Shunt voltage range is ±320mV.
    Eighth = 3,
}

impl Default for ShuntVoltageGain {
    fn default() -> Self {
        Self::Eighth
    }
}
