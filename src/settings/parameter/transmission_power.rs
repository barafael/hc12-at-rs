use core::convert::TryFrom;

/// Transmission power
#[derive(Debug, PartialEq, Eq)]
pub struct TransmissionPower(u8);

impl TransmissionPower {
    /// Construct a new TransmissionPower, if level given is valid
    pub fn new(p: u8) -> Option<Self> {
        TransmissionPower::try_from(p as u32).ok()
    }

    /// Get the power of this parameter
    pub fn power(&self) -> u8 {
        self.0
    }
}

impl Default for TransmissionPower {
    fn default() -> Self {
        Self(8)
    }
}

impl TryFrom<u32> for TransmissionPower {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            n if n > 0 && n < 9 => Ok(TransmissionPower(n as u8)),
            _ => Err(()),
        }
    }
}

impl TryFrom<i32> for TransmissionPower {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(TransmissionPower(1)),
            2 => Ok(TransmissionPower(2)),
            5 => Ok(TransmissionPower(3)),
            8 => Ok(TransmissionPower(4)),
            11 => Ok(TransmissionPower(5)),
            14 => Ok(TransmissionPower(6)),
            17 => Ok(TransmissionPower(7)),
            20 => Ok(TransmissionPower(8)),
            _ => Err(()),
        }
    }
}

impl TransmissionPower {
    /// Get the transmission power in dBm
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

    /// Get the power in milliwatt for this transmission power
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
