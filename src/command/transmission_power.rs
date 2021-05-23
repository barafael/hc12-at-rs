use crate::parameter::transmission_power::TransmissionPower;

use super::MakeCommand;

impl MakeCommand for TransmissionPower {
    fn make_command<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        match self.power() {
            1 => {
                buffer[..7].copy_from_slice(b"AT+P1\r\n");
            }
            2 => {
                buffer[..7].copy_from_slice(b"AT+P2\r\n");
            }
            3 => {
                buffer[..7].copy_from_slice(b"AT+P3\r\n");
            }
            4 => {
                buffer[..7].copy_from_slice(b"AT+P4\r\n");
            }
            5 => {
                buffer[..7].copy_from_slice(b"AT+P5\r\n");
            }
            6 => {
                buffer[..7].copy_from_slice(b"AT+P6\r\n");
            }
            7 => {
                buffer[..7].copy_from_slice(b"AT+P7\r\n");
            }
            8 => {
                buffer[..7].copy_from_slice(b"AT+P8\r\n");
            }
            _ => unreachable!(),
        }
        &buffer[..7]
    }
}
