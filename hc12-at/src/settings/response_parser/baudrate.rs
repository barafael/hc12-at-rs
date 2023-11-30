use core::convert::TryFrom;

use at_commands::parser::{CommandParser, ParseError};

use crate::settings::parameter::baudrate::BaudRate;

impl TryFrom<&[u8]> for BaudRate {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let result = CommandParser::parse(value)
            .expect_identifier(b"OK+B")
            .expect_int_parameter()
            .expect_identifier(b"\r\n")
            .finish()?;
        Ok(BaudRate::try_from(result.0).unwrap())
    }
}
