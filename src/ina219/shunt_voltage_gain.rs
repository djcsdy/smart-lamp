pub enum ShuntVoltageGain {
    /// ×1 shunt voltage gain.
    /// 
    /// Shunt voltage range is ±40mV.
    None,
    /// ÷2 shunt voltage gain.
    /// 
    /// Shunt voltage range is ±80mV.
    Half,
    /// ÷4 shunt voltage gain.
    ///
    /// Shunt voltage range is ±160mV.
    Quarter,
    /// ÷8 shunt voltage gain.
    ///
    /// Shunt voltage range is ±320mV.
    Eighth
}