use core::convert::TryFrom;

#[derive(Debug, PartialEq, Eq)]
pub struct TransmissionPower(u8);

impl TransmissionPower {
    pub fn new(p: u8) -> Option<Self> {
        TransmissionPower::try_from(p as u32).ok()
    }

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
