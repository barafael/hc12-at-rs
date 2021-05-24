use crate::settings::parameter::baudrate::BaudRate;

use super::MakeQuery;

impl MakeQuery for BaudRate {
    fn make_query(buffer: &mut [u8; 7]) {
        buffer[..7].copy_from_slice(b"AT+RB\r\n");
    }
}
