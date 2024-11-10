use crate::ina219::bus_voltage_range::BusVoltageRange;
use crate::ina219::measurement_mode::MeasurementMode;
use crate::ina219::operating_mode::OperatingMode;
use crate::ina219::shunt_voltage_gain::ShuntVoltageGain;
use defmt::Format;

#[derive(PartialEq, PartialOrd, Clone, Default, Debug, Format)]
pub struct Config {
    bus_voltage_range: BusVoltageRange,
    shunt_voltage_gain: ShuntVoltageGain,
    bus_voltage_measurement_mode: MeasurementMode,
    shunt_voltage_measurement_mode: MeasurementMode,
    operating_mode: OperatingMode,
}
