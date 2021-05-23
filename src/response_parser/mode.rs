use core::convert::TryFrom;

use at_commands::parser::{CommandParser, ParseError};

use crate::parameter::mode::Mode;

impl TryFrom<&[u8]> for Mode {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let result = CommandParser::parse(&value)
            .expect_identifier(b"OK+FU")
            .expect_int_parameter()
            .expect_identifier(b"\r\n")
            .finish();
        match result {
            Ok((1,)) => Ok(Mode::Fu1),
            Ok((2,)) => Ok(Mode::Fu2),
            Ok((3,)) => Ok(Mode::Fu3),
            Ok((4,)) => Ok(Mode::Fu4),
            Ok(_) => Err(ParseError),
            Err(e) => Err(e),
        }
    }
}
