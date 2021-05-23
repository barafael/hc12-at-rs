use crate::config::parameters::Mode;

use super::ToQuery;

impl ToQuery for Mode {
    fn to_query(buffer: &mut [u8; 16]) -> usize {
        buffer[..7].copy_from_slice(b"AT+RF\r\n");
        7
    }
}
