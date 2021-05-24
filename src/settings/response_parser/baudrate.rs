use core::convert::TryFrom;

use at_commands::parser::{CommandParser, ParseError};

use crate::settings::parameter::baudrate::BaudRate;

impl TryFrom<&[u8]> for BaudRate {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let result = CommandParser::parse(&value)
            .expect_identifier(b"OK+B")
            .expect_int_parameter()
            .expect_identifier(b"\r\n")
            .finish();
        match result {
            Ok(n) => {
                if let Ok(br) = BaudRate::try_from(n.0) {
                    Ok(br)
                } else {
                    Err(ParseError)
                }
            }
            Err(e) => Err(e),
        }
    }
}
