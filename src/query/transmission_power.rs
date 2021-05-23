use crate::config::parameters::TransmissionPower;

use super::ToQuery;

impl ToQuery for TransmissionPower {
    fn to_query(buffer: &mut [u8; 16]) -> usize {
        buffer[..7].copy_from_slice(b"AT+RP\r\n");
        7
    }
}
