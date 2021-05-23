use crate::Error;

use super::parameters::{Mode, Parameters};
use num_derive::{FromPrimitive, ToPrimitive};

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

pub trait BaudRateParameter {
    fn set_baud_rate(&mut self, rate: BaudRate) -> Result<(), Error>;
    fn get_baud_rate(&self) -> BaudRate;
    fn get_air_baud_rate(&self) -> AirBaudRate;
}

impl BaudRateParameter for Parameters {
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

    fn get_baud_rate(&self) -> BaudRate {
        self.baud_rate
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
