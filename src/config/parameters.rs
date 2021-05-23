use super::{
    baudrate::BaudRate, channel::Channel, mode::Mode, transmission_power::TransmissionPower,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Parameters {
    pub baud_rate: BaudRate,
    pub channel: Channel,
    pub power: TransmissionPower,
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
