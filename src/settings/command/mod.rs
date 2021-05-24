//! Convert a Baudrate, Channel, Mode, or Power, to a command for configuring the Hc12.

pub(crate) mod baudrate;
pub(crate) mod channel;
pub(crate) mod mode;
pub(crate) mod transmission_power;

#[cfg(test)]
mod test;

/// Convert a type to an AT command
pub trait MakeCommand {
    /// Set buffer to command for hc12
    fn make_command<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8];
}
