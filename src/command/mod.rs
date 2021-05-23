//! Convert a Baudrate, Channel, Mode, or Power, to a command for configuring the Hc12.

pub(crate) mod baudrate;
pub(crate) mod channel;
pub(crate) mod mode;
pub(crate) mod transmission_power;

/// Convert a type to an AT command
pub trait ToCommand {
    /// populate given buffer and return the size taken (0 if failed) TODO return slice
    fn to_command(&self, buffer: &mut [u8; 16]) -> usize;
}
