#![no_std]

use core::marker::PhantomData;

use at_commands::builder::CommandBuilder;

pub enum Mode {
    Fu1,
    Fu2,
    Fu3,
    Fu4,
}

pub enum Error {
    Read,
    Write,
    InvalidChannel(u8),
}

pub enum ChannelError {
    InvalidChannel(u8),
}

impl From<ChannelError> for Error {
    fn from(v: ChannelError) -> Self {
        match v {
            ChannelError::InvalidChannel(ch) => Error::InvalidChannel(ch),
        }
    }
}

pub enum BaudRate {
    Bps1200,
    Bps2400,
    Bps4800,
    Bps9600,
    Bps19200,
    Bps38400,
    Bps57600,
    Bps115200,
}

impl Default for BaudRate {
    fn default() -> Self {
        BaudRate::Bps9600
    }
}

pub struct Channel(u8);

impl Default for Channel {
    fn default() -> Self {
        Channel(1)
    }
}

impl Channel {
    pub fn get_freq_mhz(&self) -> f32 {
        433.0 + self.0 as f32 * 0.4
    }

    pub fn set_channel(&mut self, ch: u8) -> Result<(), ChannelError> {
        if ch != 0 && ch < 128 {
            self.0 = ch;
            Ok(())
        } else {
            Err(ChannelError::InvalidChannel(ch))
        }
    }
}

pub struct TransmissionPower(u8);

impl Default for TransmissionPower {
    fn default() -> Self {
        Self(8)
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

pub struct Parameters {
    pub mode: PhantomData<Mode>,
    pub baud_rate: BaudRate,
    pub channel: Channel,
    pub power: TransmissionPower,
}

#[derive(Debug, Default)]
pub struct Hc12<P, S, D> {
    set_pin: P,
    delay: D,
    serial: S,
}

pub fn ok_query() {
    let buffer: [u8; 4] = [0; 4];
    /*let _ = CommandBuilder::create_execute(&mut buffer, true)
        .named("")
        .finish()
        .unwrap();
    */
    let query = [b'A', b'T', b'\r', b'\n'];

    let buffer = [b'O', b'k', b'\r', b'\n'];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_ok() {
        let expected = b"AT\r\nOk\r\n";
    }

    #[test]
    fn bare_at_none() {
        let expected = b"AT\r\nBla\r\n";
    }

    #[test]
    fn test_channel_get_freq_default() {
        let chan = Channel::default();
        assert_eq!(433.4f32, chan.get_freq_mhz());
    }

    #[test]
    fn test_channel_get_freq_100() {
        let chan = Channel(100);
        assert_eq!(473.0f32, chan.get_freq_mhz());
    }

    #[test]
    fn test_channel_get_freq_21() {
        let chan = Channel(21);
        assert_eq!(441.4f32, chan.get_freq_mhz());
    }

    #[test]
    fn test_channel_invalid_channel() {
        let mut chan = Channel::default();
        assert!(chan.set_channel(0).is_err());
        assert!(chan.set_channel(89).is_ok());
        assert!(chan.set_channel(128).is_err());
        assert!(chan.set_channel(200).is_err());
    }
}
