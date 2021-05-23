use crate::Error;

use num_derive::{FromPrimitive, ToPrimitive};

use super::{mode::Mode, parameters::Parameters};

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

#[cfg(test)]
mod test {
    use crate::parameter::{
        baudrate::{AirBaudRate, BaudRate, BaudRateParameter},
        mode::Mode,
        parameters::Parameters,
    };

    use num_traits::{FromPrimitive, ToPrimitive};
    #[test]
    fn baud_rate_set() {
        let mut params = Parameters::default();
        params.set_baud_rate(BaudRate::Bps115200).unwrap();
        assert_eq!(params.baud_rate, BaudRate::Bps115200);
        params.mode = Mode::Fu1;
        params.set_baud_rate(BaudRate::Bps1200).unwrap();
        assert_eq!(params.baud_rate, BaudRate::Bps1200);
        assert_eq!(params.get_air_baud_rate(), AirBaudRate::Bps250000);
    }

    #[test]
    fn baud_rate_to_primitive() {
        assert_eq!(1200, BaudRate::Bps1200.to_u32().unwrap());
        assert_eq!(2400, BaudRate::Bps2400.to_u32().unwrap());
        assert_eq!(4800, BaudRate::Bps4800.to_u32().unwrap());
        assert_eq!(9600, BaudRate::Bps9600.to_u32().unwrap());
        assert_eq!(19200, BaudRate::Bps19200.to_u32().unwrap());
        assert_eq!(38400, BaudRate::Bps38400.to_u32().unwrap());
        assert_eq!(57600, BaudRate::Bps57600.to_u32().unwrap());
        assert_eq!(115200, BaudRate::Bps115200.to_u32().unwrap());
    }

    #[test]
    fn baud_rate_from_primitive() {
        assert_eq!(BaudRate::Bps1200, BaudRate::from_u32(1200).unwrap());
        assert_eq!(BaudRate::Bps2400, BaudRate::from_u32(2400).unwrap());
        assert_eq!(BaudRate::Bps4800, BaudRate::from_u32(4800).unwrap());
        assert_eq!(BaudRate::Bps9600, BaudRate::from_u32(9600).unwrap());
        assert_eq!(BaudRate::Bps19200, BaudRate::from_u32(19200).unwrap());
        assert_eq!(BaudRate::Bps38400, BaudRate::from_u32(38400).unwrap());
        assert_eq!(BaudRate::Bps57600, BaudRate::from_u32(57600).unwrap());
        assert_eq!(BaudRate::Bps115200, BaudRate::from_u32(115200).unwrap());
    }
}
