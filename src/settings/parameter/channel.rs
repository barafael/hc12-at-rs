use core::convert::TryFrom;

use num_derive::{FromPrimitive, ToPrimitive};

impl TryFrom<u8> for Channel {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Err(()),
            ch if ch > 127 => Err(()),
            n => Ok(Channel(n)),
        }
    }
}

/// Communication channel
#[derive(Debug, ToPrimitive, FromPrimitive, PartialEq, Eq)]
pub struct Channel(u8);

impl Channel {
    /// Construct a new Channel, if the given channel is valid
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
    /// Get the frequency of the channel in MHz
    pub fn get_freq_mhz(&self) -> f32 {
        433.0 + self.0 as f32 * 0.4
    }

    /// Get the channel value
    pub fn set_channel(&mut self, ch: u8) -> Result<(), crate::Error> {
        let ch = Channel::try_from(ch).map_err(|_| crate::Error::InvalidChannel)?;
        self.0 = ch.0;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::settings::parameter::channel::Channel;

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
