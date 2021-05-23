use core::convert::TryFrom;

use at_commands::parser::{CommandParser, ParseError};

use crate::parameter::transmission_power::TransmissionPower;

impl TryFrom<&[u8]> for TransmissionPower {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let result = CommandParser::parse(value)
            .expect_identifier(b"OK+RP:")
            .expect_int_parameter()
            .expect_identifier(b"dBm\r\n")
            .finish();
        match result {
            Ok(n) => match TransmissionPower::try_from(n.0) {
                Ok(p) => Ok(p),
                Err(_) => Err(ParseError),
            },
            Err(e) => Err(e),
        }
    }
}
