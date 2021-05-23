use crate::config::parameters::Mode;

use super::MakeQuery;

impl MakeQuery for Mode {
    fn make_query(buffer: &mut [u8; 16]) -> usize {
        buffer[..7].copy_from_slice(b"AT+RF\r\n");
        7
    }
}
