#[repr(u8)]
pub enum Param {
    BaudRate = b"B"[0],
    Channel = b"C"[0],
    Mode = b"F"[0],
    Power = b"P"[0],
}

pub trait ToQuery {
    fn to_query(&self, buffer: &mut [u8; 16]) -> usize;
}

impl ToQuery for Param {
    fn to_query(&self, buffer: &mut [u8; 16]) -> usize {
        match self {
            Param::BaudRate => buffer[..7].copy_from_slice(b"AT+RB\r\n"),
            Param::Channel => buffer[..7].copy_from_slice(b"AT+RC\r\n"),
            Param::Mode => buffer[..7].copy_from_slice(b"AT+RF\r\n"),
            Param::Power => buffer[..7].copy_from_slice(b"AT+RP\r\n"),
        }
        7
    }
}
