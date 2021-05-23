use at_commands::builder::CommandBuilder;

use crate::config::parameters::BaudRate;

use super::ToCommand;

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
