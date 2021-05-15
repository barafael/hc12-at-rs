use super::parameters::VERSION_QUERY;

#[repr(u8)]
pub enum Param {
    BaudRate,
    Channel,
    Mode,
    Power,
    Version,
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
            Param::Version => {
                buffer[..6].copy_from_slice(&VERSION_QUERY);
                return 6;
            }
        }
        7
    }
}
