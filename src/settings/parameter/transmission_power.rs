use core::convert::TryFrom;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

/// Transmission power
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum TransmissionPower {
    /// Power -1 dBm
    One = 1,
    /// Power 2 dBm
    Two = 2,
    /// Power 5 dBm
    Three = 3,
    /// Power 8 dBm
    Four = 4,
    /// Power 11 dBm
    Five = 5,
    /// Power 14 dBm
    Six = 6,
    /// Power 17 dBm
    Seven = 7,
    /// Power 20 dBm
    Eight = 8,
}

impl TransmissionPower {
    /// Construct a new TransmissionPower, if level given is valid
    pub fn new(p: u8) -> Option<Self> {
        TransmissionPower::from_u8(p)
    }
}

impl Default for TransmissionPower {
    fn default() -> Self {
        Self::Eight
    }
}

impl TryFrom<u8> for TransmissionPower {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(value as i32)
    }
}

impl TryFrom<i32> for TransmissionPower {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(TransmissionPower::One),
            2 => Ok(TransmissionPower::Two),
            5 => Ok(TransmissionPower::Three),
            8 => Ok(TransmissionPower::Four),
            11 => Ok(TransmissionPower::Five),
            14 => Ok(TransmissionPower::Six),
            17 => Ok(TransmissionPower::Seven),
            20 => Ok(TransmissionPower::Eight),
            _ => Err(()),
        }
    }
}

impl TransmissionPower {
    /// Get the transmission power in dBm
    pub fn get_power_dbm(&self) -> i8 {
        match &self {
            TransmissionPower::One => -1,
            TransmissionPower::Two => 2,
            TransmissionPower::Three => 5,
            TransmissionPower::Four => 8,
            TransmissionPower::Five => 11,
            TransmissionPower::Six => 14,
            TransmissionPower::Seven => 17,
            TransmissionPower::Eight => 20,
        }
    }

    /// Get the power in milliwatt for this transmission power
    pub fn get_power_milliwatt(&self) -> f32 {
        match &self {
            TransmissionPower::One => 0.79,
            TransmissionPower::Two => 1.58,
            TransmissionPower::Three => 3.16,
            TransmissionPower::Four => 6.31,
            TransmissionPower::Five => 12.59,
            TransmissionPower::Six => 25.12,
            TransmissionPower::Seven => 50.12,
            TransmissionPower::Eight => 100.0,
        }
    }
}

#[cfg(test)]
mod test {
    use core::convert::TryFrom;

    use crate::settings::parameter::transmission_power::TransmissionPower;

    #[test]
    fn parse_power_from_i32() {
        let dbm = [-1, 2, 5, 8, 11, 14, 17, 20];

        for (i, dbm) in dbm.iter().enumerate() {
            let expected = TransmissionPower::new((i as u8) + 1).unwrap();
            let power = TransmissionPower::try_from(*dbm).unwrap();
            assert_eq!(expected, power);
        }
    }
}
