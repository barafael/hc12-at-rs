//! Convert a Baudrate, Channel, Mode, or Power, to a command for configuring the Hc12.

pub(crate) mod baudrate;
pub(crate) mod channel;
pub(crate) mod mode;
pub(crate) mod transmission_power;

/// Convert a type to an AT command
pub trait MakeCommand {
    /// populate given buffer and return the size taken (0 if failed) TODO return slice
    fn make_command(&self, buffer: &mut [u8; 16]) -> usize;
}
