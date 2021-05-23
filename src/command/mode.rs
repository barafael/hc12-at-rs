use crate::parameter::mode::Mode;

use super::MakeCommand;

impl MakeCommand for Mode {
    fn make_command(&self, buffer: &mut [u8; 16]) -> usize {
        match self {
            Mode::Fu1 => {
                buffer[0..8].copy_from_slice(b"AT+FU1\r\n");
                8
            }
            Mode::Fu2 => {
                buffer[0..8].copy_from_slice(b"AT+FU2\r\n");
                8
            }
            Mode::Fu3 => {
                buffer[0..8].copy_from_slice(b"AT+FU3\r\n");
                8
            }
            Mode::Fu4 => {
                buffer[0..8].copy_from_slice(b"AT+FU4\r\n");
                8
            }
        }
    }
}
