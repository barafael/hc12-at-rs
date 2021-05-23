use crate::config::channel::Channel;

use super::MakeQuery;

impl MakeQuery for Channel {
    fn make_query(buffer: &mut [u8; 16]) -> usize {
        buffer[..7].copy_from_slice(b"AT+RC\r\n");
        7
    }
}
