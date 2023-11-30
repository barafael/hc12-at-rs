use core::convert::TryFrom;

use at_commands::parser::{CommandParser, ParseError};

use crate::settings::parameter::transmission_power::TransmissionPower;

impl TryFrom<&[u8]> for TransmissionPower {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let result = CommandParser::parse(value)
            .expect_identifier(b"OK+RP:")
            .expect_int_parameter()
            .expect_identifier(b"dBm\r\n")
            .finish()?;
        Ok(TransmissionPower::try_from(result.0).unwrap())
    }
}
