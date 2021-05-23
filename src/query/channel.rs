use crate::config::parameters::Channel;

use super::ToQuery;

impl ToQuery for Channel {
    fn to_query(buffer: &mut [u8; 16]) -> usize {
            buffer[..7].copy_from_slice(b"AT+RC\r\n");
            7
    }
}
