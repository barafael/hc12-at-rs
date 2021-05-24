use crate::settings::parameter::channel::Channel;

use super::MakeQuery;

impl MakeQuery for Channel {
    fn make_query(buffer: &mut [u8; 7]) {
        buffer[..7].copy_from_slice(b"AT+RC\r\n");
    }
}
