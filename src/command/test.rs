use crate::{
    command::MakeCommand,
    parameter::{
        baudrate::BaudRate, channel::Channel, mode::Mode, transmission_power::TransmissionPower,
    },
};

#[test]
fn set_baudrate_command() {
    let mut buffer = [0u8; 16];
    let baudrate = BaudRate::Bps115200;
    let c = baudrate.make_command(&mut buffer);
    assert_eq!(b"AT+B115200\r\n", c)
}

#[test]
fn set_channel_command() {
    let mut buffer = [0u8; 16];
    for i in 1..128 {
        let channel = Channel::new(i).unwrap();
        let c = channel.make_command(&mut buffer);
        assert_eq!(format!("AT+C{:0width$}\r\n", i, width = 3).as_bytes(), c);
    }
}

#[test]
fn set_mode_command() {
    let mut buffer = [0u8; 16];
    let mode = Mode::Fu1;
    let c = mode.make_command(&mut buffer);
    assert_eq!(b"AT+FU1\r\n", c);
}

#[test]
fn set_power_command() {
    let mut buffer = [0u8; 16];
    let power = TransmissionPower(8);
    let c = power.make_command(&mut buffer);
    assert_eq!(b"AT+P8\r\n", c);
}
