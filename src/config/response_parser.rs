//! Parse responses from a Hc12

use core::convert::TryFrom;

use at_commands::parser::{CommandParser, ParseError};

use super::parameters::{BaudRate, Channel, Mode, Parameters, TransmissionPower};

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

impl TryFrom<&[u8]> for Channel {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let result = CommandParser::parse(value)
            .expect_identifier(b"OK+RC")
            .expect_int_parameter()
            .expect_identifier(b"\r\n")
            .finish()
            .unwrap();
        let byte: u8 = result.0 as u8; // TODO fallible cast
        match Channel::try_from(byte) {
            Ok(ch) => Ok(ch),
            Err(_) => Err(ParseError),
        }
    }
}

impl TryFrom<&[u8]> for Parameters {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Parameters::default())
    }
}
