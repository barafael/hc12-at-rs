use core::convert::TryFrom;

use crate::parameter::{
    baudrate::BaudRate, channel::Channel, mode::Mode, transmission_power::TransmissionPower,
};

#[test]
fn parse_baudrate() {
    let response = b"OK+B9600\r\n";
    let rate = BaudRate::try_from(&response[..]).unwrap();
    assert_eq!(BaudRate::Bps9600, rate);
    let response = b"OK+B115200\r\n";
    let rate = BaudRate::try_from(&response[..]).unwrap();
    assert_eq!(BaudRate::Bps115200, rate);
    let response = b"OK+B1200\r\n";
    let rate = BaudRate::try_from(&response[..]).unwrap();
    assert_eq!(BaudRate::Bps1200, rate);
}

#[test]
fn parse_channel() {
    let response = b"OK+RC005\r\n";
    let channel = Channel(5);
    assert_eq!(Channel::try_from(&response[..]).unwrap(), channel);
    let response = b"OK+RC100\r\n";
    let channel = Channel(100);
    assert_eq!(Channel::try_from(&response[..]).unwrap(), channel);
    let response = b"OK+RC099\r\n";
    let channel = Channel(99);
    assert_eq!(Channel::try_from(&response[..]).unwrap(), channel);
}

#[test]
fn parse_mode() {
    let response = b"OK+FU1\r\n";
    let mode = Mode::Fu1;
    assert_eq!(Mode::try_from(&response[..]).unwrap(), mode);
    let response = b"OK+FU2\r\n";
    let mode = Mode::Fu2;
    assert_eq!(Mode::try_from(&response[..]).unwrap(), mode);
    let response = b"OK+FU4\r\n";
    let mode = Mode::Fu4;
    assert_eq!(Mode::try_from(&response[..]).unwrap(), mode);
}

#[test]
fn parse_power() {
    let response = b"OK+RP:-1dBm\r\n";
    let power = TransmissionPower(1);
    assert_eq!(TransmissionPower::try_from(&response[..]).unwrap(), power);
    let response = b"OK+RP:+20dBm\r\n";
    let power = TransmissionPower(8);
    assert_eq!(TransmissionPower::try_from(&response[..]).unwrap(), power);
    let response = b"OK+RP:+5dBm\r\n";
    let power = TransmissionPower(3);
    assert_eq!(TransmissionPower::try_from(&response[..]).unwrap(), power);
}
