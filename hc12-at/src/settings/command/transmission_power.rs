use crate::settings::parameter::transmission_power::TransmissionPower;

use super::MakeCommand;

impl MakeCommand for TransmissionPower {
    fn make_command<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        match self {
            TransmissionPower::One => {
                buffer[..7].copy_from_slice(b"AT+P1\r\n");
            }
            TransmissionPower::Two => {
                buffer[..7].copy_from_slice(b"AT+P2\r\n");
            }
            TransmissionPower::Three => {
                buffer[..7].copy_from_slice(b"AT+P3\r\n");
            }
            TransmissionPower::Four => {
                buffer[..7].copy_from_slice(b"AT+P4\r\n");
            }
            TransmissionPower::Five => {
                buffer[..7].copy_from_slice(b"AT+P5\r\n");
            }
            TransmissionPower::Six => {
                buffer[..7].copy_from_slice(b"AT+P6\r\n");
            }
            TransmissionPower::Seven => {
                buffer[..7].copy_from_slice(b"AT+P7\r\n");
            }
            TransmissionPower::Eight => {
                buffer[..7].copy_from_slice(b"AT+P8\r\n");
            }
        }
        &buffer[..7]
    }
}
