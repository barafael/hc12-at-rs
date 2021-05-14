use core::convert::TryFrom;

use at_commands::parser::CommandParser;

use super::{BaudRate, Channel, Parameters};

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
    type Error = ();

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
                    Err(())
                }
            }
            Err(e) => Err(()),
        }
    }
}

impl TryFrom<&[u8; 10]> for Channel {
    type Error = ();

    fn try_from(value: &[u8; 10]) -> Result<Self, Self::Error> {
        let result = CommandParser::parse(value)
            .expect_identifier(b"OK+RC")
            .expect_int_parameter()
            .expect_identifier(b"\r\n")
            .finish()
            .unwrap();
        let byte: u8 = result.0 as u8; // TODO fallible cast
        if let Ok(ch) = Channel::try_from(byte) {
            Ok(ch)
        } else {
            Err(())
        }
    }
}

impl TryFrom<&[u8]> for Parameters {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Parameters::default())
    }
}
