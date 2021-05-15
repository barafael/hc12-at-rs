use core::convert::TryFrom;

use crate::Error;

use num_derive::{FromPrimitive, ToPrimitive};

pub mod command;
pub mod parser;
pub mod query;

#[cfg(test)]
mod test;

trait SetBaudRate {
    fn set_baud_rate(&mut self, rate: BaudRate) -> Result<(), Error>;
    fn get_air_baud_rate(&self) -> AirBaudRate;
}

#[derive(Debug)]
pub enum Mode {
    Fu1,
    Fu2,
    Fu3,
    Fu4,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Fu3
    }
}

impl SetBaudRate for Parameters {
    fn set_baud_rate(&mut self, rate: BaudRate) -> Result<(), Error> {
        match self.mode {
            Mode::Fu1 => {
                self.baud_rate = rate;
                Ok(())
            }
            Mode::Fu2 => match rate {
                BaudRate::Bps1200 | BaudRate::Bps2400 | BaudRate::Bps4800 => {
                    self.baud_rate = rate;
                    Ok(())
                }
                _ => Err(Error::InvalidBaudRate),
            },
            Mode::Fu3 => {
                self.baud_rate = rate;
                Ok(())
            }
            Mode::Fu4 => {
                todo!()
            }
        }
    }

    fn get_air_baud_rate(&self) -> AirBaudRate {
        match self.mode {
            Mode::Fu1 => AirBaudRate::Bps250000,
            Mode::Fu2 => AirBaudRate::Bps250000,
            Mode::Fu3 => match self.baud_rate {
                BaudRate::Bps1200 => AirBaudRate::Bps5000,
                BaudRate::Bps2400 => AirBaudRate::Bps5000,
                BaudRate::Bps4800 => AirBaudRate::Bps15000,
                BaudRate::Bps9600 => AirBaudRate::Bps15000,
                BaudRate::Bps19200 => AirBaudRate::Bps58000,
                BaudRate::Bps38400 => AirBaudRate::Bps58000,
                BaudRate::Bps57600 => AirBaudRate::Bps236000,
                BaudRate::Bps115200 => AirBaudRate::Bps236000,
            },
            Mode::Fu4 => {
                todo!()
            }
        }
    }
}

pub fn get_wireless_sensitivity_dbm(air_rate: AirBaudRate) -> i32 {
    match air_rate {
        AirBaudRate::Bps5000 => -117,
        AirBaudRate::Bps15000 => -117,
        AirBaudRate::Bps58000 => -112,
        AirBaudRate::Bps236000 => -100,
        AirBaudRate::Bps250000 => -100, // TODO Datasheet doesn't say; extrapolate?
    }
}

pub enum ChannelError {
    InvalidChannel(u8),
}

impl From<ChannelError> for Error {
    fn from(v: ChannelError) -> Self {
        match v {
            ChannelError::InvalidChannel(ch) => Error::InvalidChannel(ch),
        }
    }
}

impl TryFrom<u8> for Channel {
    type Error = ChannelError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Err(ChannelError::InvalidChannel(0)),
            ch if ch > 127 => Err(ChannelError::InvalidChannel(ch)),
            n => Ok(Channel(n)),
        }
    }
}

#[derive(Debug, ToPrimitive, FromPrimitive, PartialEq, Eq)]
pub struct Channel(u8);

impl Default for Channel {
    fn default() -> Self {
        Channel(1)
    }
}

impl Channel {
    pub fn get_freq_mhz(&self) -> f32 {
        433.0 + self.0 as f32 * 0.4
    }

    pub fn set_channel(&mut self, ch: u8) -> Result<(), ChannelError> {
        if ch != 0 && ch < 128 {
            self.0 = ch;
            Ok(())
        } else {
            Err(ChannelError::InvalidChannel(ch))
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum BaudRate {
    Bps1200 = 1200,
    Bps2400 = 2400,
    Bps4800 = 4800,
    Bps9600 = 9600,
    Bps19200 = 19200,
    Bps38400 = 38400,
    Bps57600 = 57600,
    Bps115200 = 115200,
}

#[derive(Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum AirBaudRate {
    Bps5000 = 5000,
    Bps15000 = 15000,
    Bps58000 = 58000,
    Bps236000 = 236000,
    Bps250000 = 250000,
}

impl Default for BaudRate {
    fn default() -> Self {
        BaudRate::Bps9600
    }
}

#[derive(Debug)]
pub struct TransmissionPower(u8);

impl Default for TransmissionPower {
    fn default() -> Self {
        Self(8)
    }
}

impl TransmissionPower {
    pub fn get_power_dbm(&self) -> i8 {
        match self.0 {
            1 => -1,
            2 => 2,
            3 => 5,
            4 => 8,
            5 => 11,
            6 => 14,
            7 => 17,
            8 => 20,
            _ => unreachable!(),
        }
    }

    pub fn get_power_milliwatt(&self) -> f32 {
        match self.0 {
            1 => 0.79,
            2 => 1.58,
            3 => 3.16,
            4 => 6.31,
            5 => 12.59,
            6 => 25.12,
            7 => 50.12,
            8 => 100.0,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
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

pub(crate) const OK_QUERY: [u8; 4] = *b"AT\r\n";
pub(crate) const OK_RESPONSE: [u8; 4] = *b"OK\r\n";
pub(crate) const SLEEP_COMMAND: [u8; 10] = *b"AT+SLEEP\r\n";
pub(crate) const REVISION_QUERY: [u8; 6] = *b"AT+V\r\n";
pub(crate) const RESET_SETTINGS_COMMAND: [u8; 12] = *b"AT+DEFAULT\r\n";
pub(crate) const UPDATE_COMMAND: [u8; 11] = *b"AT+UPDATE\r\n";
