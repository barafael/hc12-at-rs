use core::convert::TryFrom;

use crate::Error;

use num_derive::{FromPrimitive, ToPrimitive};

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

impl TryFrom<u8> for Channel {
    type Error = ChannelError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Err(ChannelError::InvalidChannel(0)),
            ch if ch > 127 => Err(ChannelError::InvalidChannel(ch)),
            n => Ok(Channel(n)),
        }
    }
}

#[derive(Debug, ToPrimitive, FromPrimitive, PartialEq, Eq)]
pub struct Channel(u8);

impl Channel {
    pub fn new(ch: u8) -> Option<Self> {
        Self::try_from(ch).ok()
    }
}

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

#[cfg(test)]
mod test {
    use crate::parameter::channel::Channel;

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
