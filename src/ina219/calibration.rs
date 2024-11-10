use defmt::Format;

#[derive(PartialEq, PartialOrd, Clone, Debug, Format)]
pub struct Calibration {
    value: u16,
    shunt_resistance_ohms: f32,
}

impl Calibration {
    pub fn from_maximum_expected_current_and_shunt_resistance(
        maximum_expected_current_amps: f32,
        shunt_resistance_ohms: f32,
    ) -> Self {
        Self::from_current_lsb_and_shunt_resistance(
            maximum_expected_current_amps / 32768.0,
            shunt_resistance_ohms,
        )
    }

    pub fn from_current_lsb_and_shunt_resistance(
        current_lsb_amps: f32,
        shunt_resistance_ohms: f32,
    ) -> Self {
        let calibration = (0.04096 / (current_lsb_amps * shunt_resistance_ohms)).max(0.0);
        Self {
            value: calibration as u16,
            shunt_resistance_ohms,
        }
    }
}
