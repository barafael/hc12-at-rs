use core::convert::TryInto;

use at_commands::builder::CommandBuilder;
use num_traits::ToPrimitive;

use super::{BaudRate, Channel, Mode};

pub trait ToCommand {
    fn to_command(&self, buffer: &mut [u8; 16]) -> usize;
}

impl From<&BaudRate> for &[u8] {
    fn from(r: &BaudRate) -> Self {
        match r {
            BaudRate::Bps1200 => b"1200",
            BaudRate::Bps2400 => b"2400",
            BaudRate::Bps4800 => b"4800",
            BaudRate::Bps9600 => b"9600",
            BaudRate::Bps19200 => b"19200",
            BaudRate::Bps38400 => b"38400",
            BaudRate::Bps57600 => b"57600",
            BaudRate::Bps115200 => b"115200",
        }
    }
}

impl ToCommand for BaudRate {
    fn to_command(&self, buffer: &mut [u8; 16]) -> usize {
        let mut format_buf = [0u8; 8];
        let num: &[u8] = self.into();
        format_buf[0..2].copy_from_slice(b"+B");
        format_buf[2..2 + num.len()].copy_from_slice(num);
        let command = core::str::from_utf8(&format_buf[..2 + num.len()]).unwrap();
        let result = CommandBuilder::create_execute(buffer, true)
            .named(command)
            .finish()
            .unwrap();
        result.len()
    }
}

impl From<&Channel> for [u8; 3] {
    fn from(c: &Channel) -> Self {
        fn base_10_bytes_padded(mut n: u8, buf: &mut [u8]) -> &[u8] {
            if n == 0 {
                return b"0";
            }
            for i in buf.iter_mut() {
                if n > 0 {
                    *i = (n % 10) as u8 + b'0';
                    n /= 10;
                } else {
                    *i = b"0"[0];
                }
            }
            buf.reverse();
            buf
        }
        let mut buf = [0u8; 3];
        let bytes = base_10_bytes_padded(c.to_u8().unwrap(), &mut buf);
        bytes.try_into().unwrap()
    }
}

impl ToCommand for Channel {
    fn to_command(&self, buffer: &mut [u8; 16]) -> usize {
        let mut format_buf = [0u8; 5];
        let num: [u8; 3] = self.into();
        format_buf[0..2].copy_from_slice(b"+C");
        format_buf[2..5].copy_from_slice(&num);
        let command = core::str::from_utf8(&format_buf[..2 + num.len()]).unwrap();
        let result = CommandBuilder::create_execute(buffer, true)
            .named(command)
            .finish()
            .unwrap();
        result.len()
    }
}

impl ToCommand for Mode {
    fn to_command(&self, buffer: &mut [u8; 16]) -> usize {
        match self {
            Mode::Fu1 => {
                buffer[0..8].copy_from_slice(b"AT+FU1\r\n");
                8
            }
            Mode::Fu2 => {
                buffer[0..8].copy_from_slice(b"AT+FU2\r\n");
                8
            }
            Mode::Fu3 => {
                buffer[0..8].copy_from_slice(b"AT+FU3\r\n");
                8
            }
            Mode::Fu4 => {
                buffer[0..8].copy_from_slice(b"AT+FU4\r\n");
                8
            }
        }
    }
}
