use at_commands::builder::CommandBuilder;
use core::convert::TryInto;

use crate::parameter::channel::Channel;

use super::MakeCommand;

use num_traits::ToPrimitive;

impl MakeCommand for Channel {
    fn make_command(&self, buffer: &mut [u8; 16]) -> usize {
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
