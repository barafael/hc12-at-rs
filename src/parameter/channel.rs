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
pub struct Channel(pub(crate) u8);

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
