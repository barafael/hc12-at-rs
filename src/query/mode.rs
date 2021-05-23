use crate::parameter::mode::Mode;

use super::MakeQuery;

impl MakeQuery for Mode {
    fn make_query(buffer: &mut [u8; 7]) {
        buffer[..7].copy_from_slice(b"AT+RF\r\n");
    }
}
