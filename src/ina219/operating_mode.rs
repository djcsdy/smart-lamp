use defmt::Format;

#[repr(u8)]
#[derive(PartialEq, PartialOrd, Clone, Hash, Debug, Format)]
pub enum OperatingMode {
    PowerDown = 0,
    ShuntVoltageTriggered = 1,
    BusVoltageTriggered = 2,
    ShuntAndBusVoltageTriggered = 3,
    Disabled = 4,
    ShuntVoltageContinuous = 5,
    BusVoltageContinuous = 6,
    ShuntAndBusVoltageContinuous = 7
}