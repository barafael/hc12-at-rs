use crate::config::baudrate::BaudRate;

use super::MakeQuery;

impl MakeQuery for BaudRate {
    fn make_query(buffer: &mut [u8; 16]) -> usize {
        buffer[..7].copy_from_slice(b"AT+RB\r\n");
        7
    }
}
