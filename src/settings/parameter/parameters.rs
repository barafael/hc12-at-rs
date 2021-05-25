use super::{
    baudrate::BaudRate, channel::Channel, mode::Mode, transmission_power::TransmissionPower,
};

/// All hc12 parameters
#[derive(Debug, PartialEq, Eq)]
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

impl Default for Parameters {
    fn default() -> Self {
        Self {
            baud_rate: BaudRate::default(),
            channel: Channel::default(),
            power: TransmissionPower::default(),
            mode: Mode::default(),
        }
    }
}
