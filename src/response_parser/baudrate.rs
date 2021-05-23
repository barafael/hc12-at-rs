use core::convert::TryFrom;

use at_commands::parser::{CommandParser, ParseError};

use crate::config::baudrate::BaudRate;

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
