use core::convert::TryFrom;

use crate::Error;

use num_derive::{FromPrimitive, ToPrimitive};

use super::{mode::Mode, parameters::Parameters};

/// Baud rate of HC-12
#[repr(u32)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum BaudRate {
    /// 1200 bauds per second
    Bps1200 = 1200,

    /// 2400 bauds per second
    Bps2400 = 2400,

    /// 4800 bauds per second
    Bps4800 = 4800,

    #[default]
    /// 9600 bauds per second
    Bps9600 = 9600,

    /// 19200 bauds per second
    Bps19200 = 19200,

    /// 38400 bauds per second
    Bps38400 = 38400,

    /// 57600 bauds per second
    Bps57600 = 57600,

    /// 115200 bauds per second
    Bps115200 = 115200,
}

/// Baud rate in the air
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum AirBaudRate {
    /// 5000 bauds per second
    Bps5000 = 5000,

    /// 15000 bauds per second
    Bps15000 = 15000,

    /// 58000 bauds per second
    Bps58000 = 58000,

    /// 236000 bauds per second
    Bps236000 = 236000,

    /// 250000 bauds per second
    Bps250000 = 250000,
}

impl TryFrom<i32> for BaudRate {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1200 => Ok(BaudRate::Bps1200),
            2400 => Ok(BaudRate::Bps2400),
            4800 => Ok(BaudRate::Bps4800),
            9600 => Ok(BaudRate::Bps9600),
            19200 => Ok(BaudRate::Bps19200),
            38400 => Ok(BaudRate::Bps38400),
            57600 => Ok(BaudRate::Bps57600),
            115200 => Ok(BaudRate::Bps115200),
            _ => Err(()),
        }
    }
}

impl Parameters {
    /// Set the baud rate of the parameters
    pub fn set_baud_rate(&mut self, rate: BaudRate) -> Result<(), Error> {
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

    /// Try to get the (depends on serial baud rate + info from datasheet)
    pub fn get_air_baud_rate(&self) -> AirBaudRate {
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

impl AirBaudRate {
    /// Get the wireless sensitivity in dbm of this air baud rate
    pub fn get_wireless_sensitivity_dbm(&self) -> i32 {
        match self {
            AirBaudRate::Bps5000 => -117,
            AirBaudRate::Bps15000 => -117,
            AirBaudRate::Bps58000 => -112,
            AirBaudRate::Bps236000 => -100,
            AirBaudRate::Bps250000 => -100, // TODO Datasheet doesn't say; extrapolate?
        }
    }
}

#[cfg(test)]
mod test {
    use core::convert::TryFrom;

    use crate::settings::parameter::{
        baudrate::{AirBaudRate, BaudRate},
        mode::Mode,
        parameters::Parameters,
        transmission_power::TransmissionPower,
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

        params.mode = Mode::Fu2;
        params.set_baud_rate(BaudRate::Bps1200).unwrap();

        assert!(params.set_baud_rate(BaudRate::Bps115200).is_err());
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

    #[test]
    fn parse_baudrate_from_i32() {
        let baudrates = [1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200];
        let expected = [
            BaudRate::Bps1200,
            BaudRate::Bps2400,
            BaudRate::Bps4800,
            BaudRate::Bps9600,
            BaudRate::Bps19200,
            BaudRate::Bps38400,
            BaudRate::Bps57600,
            BaudRate::Bps115200,
        ];
        let result: Vec<BaudRate> = baudrates
            .iter()
            .map(|x| BaudRate::try_from(*x).unwrap())
            .collect();
        assert_eq!(&expected, &result[..]);
    }

    #[test]
    fn get_air_baud_rate() {
        let air_rates = [
            AirBaudRate::Bps5000,
            AirBaudRate::Bps5000,
            AirBaudRate::Bps15000,
            AirBaudRate::Bps15000,
            AirBaudRate::Bps58000,
            AirBaudRate::Bps58000,
            AirBaudRate::Bps236000,
            AirBaudRate::Bps236000,
        ];

        let rates = [
            BaudRate::Bps1200,
            BaudRate::Bps2400,
            BaudRate::Bps4800,
            BaudRate::Bps9600,
            BaudRate::Bps19200,
            BaudRate::Bps38400,
            BaudRate::Bps57600,
            BaudRate::Bps115200,
        ];

        let mut params = Parameters::default();

        for (r, a) in rates.iter().zip(air_rates.iter()) {
            params.baud_rate = *r;
            assert_eq!(*a, params.get_air_baud_rate());
        }
    }

    #[test]
    fn air_baudrate_fu2() {
        let mut params = Parameters::default();
        params.mode = Mode::Fu2;

        assert_eq!(AirBaudRate::Bps250000, params.get_air_baud_rate());
    }

    #[test]
    fn get_wireless_sensitivity_dbm() {
        let rate = AirBaudRate::Bps5000;
        assert_eq!(-117, rate.get_wireless_sensitivity_dbm());
        let rate = AirBaudRate::Bps15000;
        assert_eq!(-117, rate.get_wireless_sensitivity_dbm());
        let rate = AirBaudRate::Bps58000;
        assert_eq!(-112, rate.get_wireless_sensitivity_dbm());
        let rate = AirBaudRate::Bps236000;
        assert_eq!(-100, rate.get_wireless_sensitivity_dbm());
        let rate = AirBaudRate::Bps250000;
        assert_eq!(-100, rate.get_wireless_sensitivity_dbm());
    }

    #[test]
    fn get_power_dbm() {
        let dbm = [-1, 2, 5, 8, 11, 14, 17, 20];
        for (i, dbm) in dbm.iter().enumerate() {
            let power = TransmissionPower::new((i + 1) as u8).unwrap();
            assert_eq!(power.get_power_dbm(), *dbm);
        }
    }

    #[test]
    fn get_power_milliwatt() {
        let m_w = [0.79, 1.58, 3.16, 6.31, 12.59, 25.12, 50.12, 100.0];
        for (i, m_w) in m_w.iter().enumerate() {
            let power = TransmissionPower::new((i + 1) as u8).unwrap();
            assert_eq!(power.get_power_milliwatt(), *m_w);
        }
    }
}
