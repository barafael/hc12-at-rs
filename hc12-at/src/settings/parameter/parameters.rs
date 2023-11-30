use super::{
    baudrate::BaudRate, channel::Channel, mode::Mode, transmission_power::TransmissionPower,
};

/// All hc12 parameters
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Parameters {
    /// Baud rate
    pub baud_rate: BaudRate,
    /// Communication channel
    pub channel: Channel,
    /// Transmission power
    pub power: TransmissionPower,
    /// Operation mode
    pub mode: Mode,
}
