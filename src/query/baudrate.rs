use crate::config::parameters::BaudRate;

use super::ToQuery;

impl ToQuery for BaudRate {
    fn to_query(buffer: &mut [u8; 16]) -> usize {
        buffer[..7].copy_from_slice(b"AT+RB\r\n");
        7
    }
}
