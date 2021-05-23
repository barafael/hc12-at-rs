use crate::parameter::transmission_power::TransmissionPower;

use super::MakeQuery;

impl MakeQuery for TransmissionPower {
    fn make_query(buffer: &mut [u8; 16]) -> usize {
        buffer[..7].copy_from_slice(b"AT+RP\r\n");
        7
    }
}
