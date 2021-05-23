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
    let modes = [Mode::Fu1, Mode::Fu2, Mode::Fu3, Mode::Fu4];
    for (index, mode) in modes.iter().enumerate() {
        let expected = format!("AT+FU{}\r\n", index + 1);
        let com = mode.make_command(&mut buffer);
        assert_eq!(expected.as_bytes(), com);
    }
}

#[test]
fn set_power_command() {
    let mut buffer = [0u8; 16];
    for power in 1..9 {
        let p = TransmissionPower::new(power).unwrap();
        let c = p.make_command(&mut buffer);
        let expected = format!("AT+P{}\r\n", power);
        assert_eq!(expected.as_bytes(), c);
    }
}
