use core::convert::TryFrom;

use at_commands::parser::{CommandParser, ParseError};

use crate::settings::parameter::channel::Channel;

impl TryFrom<&[u8]> for Channel {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let result = CommandParser::parse(value)
            .expect_identifier(b"OK+RC")
            .expect_int_parameter()
            .expect_identifier(b"\r\n")
            .finish()
            .unwrap();
        let byte: u8 = result.0 as u8;
        Channel::try_from(byte).map_err(|_| ParseError)
    }
}
