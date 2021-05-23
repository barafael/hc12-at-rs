use crate::parameter::mode::Mode;

use super::MakeCommand;

impl MakeCommand for Mode {
    fn make_command<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        match self {
            Mode::Fu1 => {
                buffer[0..8].copy_from_slice(b"AT+FU1\r\n");
                &buffer[..8]
            }
            Mode::Fu2 => {
                buffer[0..8].copy_from_slice(b"AT+FU2\r\n");
                &buffer[..8]
            }
            Mode::Fu3 => {
                buffer[0..8].copy_from_slice(b"AT+FU3\r\n");
                &buffer[..8]
            }
            Mode::Fu4 => {
                buffer[0..8].copy_from_slice(b"AT+FU4\r\n");
                &buffer[..8]
            }
        }
    }
}
