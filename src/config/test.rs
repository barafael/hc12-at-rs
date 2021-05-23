use core::convert::TryFrom;

use crate::{
    command::*,
    config::{
        baudrate::{AirBaudRate, BaudRate, BaudRateParameter},
        channel::Channel,
    },
    query::MakeQuery,
};

use crate::config::parameters::{Mode, Parameters, TransmissionPower};

use num_traits::{FromPrimitive, ToPrimitive};

#[test]
fn set_baudrate_command() {
    let mut buffer = [0u8; 16];
    let baudrate = BaudRate::Bps115200;
    let n = baudrate.make_command(&mut buffer);
    assert_eq!(b"AT+B115200\r\n", &buffer[0..n])
}

#[test]
fn set_channel_command() {
    let mut buffer = [0u8; 16];
    for i in 1..128 {
        let channel = Channel(i);
        let n = channel.make_command(&mut buffer);
        assert_eq!(
            format!("AT+C{:0width$}\r\n", i, width = 3).as_bytes(),
            &buffer[0..n]
        );
    }
}

#[test]
fn set_mode_command() {
    let mut buffer = [0u8; 16];
    let mode = Mode::Fu1;
    let n = mode.make_command(&mut buffer);
    assert_eq!(b"AT+FU1\r\n", &buffer[0..n])
}

#[test]
fn set_power_command() {
    let mut buffer = [0u8; 16];
    let power = TransmissionPower(8);
    let n = power.make_command(&mut buffer);
    assert_eq!(b"AT+P8\r\n", &buffer[0..n])
}

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

#[test]
fn query_single_param() {
    let mut buffer = [0u8; 16];
    let n = BaudRate::make_query(&mut buffer);
    assert_eq!(b"AT+RB\r\n", &buffer[..n]);
    let n = Channel::make_query(&mut buffer);
    assert_eq!(b"AT+RC\r\n", &buffer[..n]);
    let n = Mode::make_query(&mut buffer);
    assert_eq!(b"AT+RF\r\n", &buffer[..n]);
    let n = TransmissionPower::make_query(&mut buffer);
    assert_eq!(b"AT+RP\r\n", &buffer[..n]);
}

#[test]
fn test_channel_get_freq_default() {
    let chan = Channel::default();
    assert_eq!(433.4f32, chan.get_freq_mhz());
}

#[test]
fn test_channel_get_freq_100() {
    let chan = Channel(100);
    assert_eq!(473.0f32, chan.get_freq_mhz());
}

#[test]
fn test_channel_get_freq_21() {
    let chan = Channel(21);
    assert_eq!(441.4f32, chan.get_freq_mhz());
}

#[test]
fn test_channel_invalid_channel() {
    let mut chan = Channel::default();
    assert!(chan.set_channel(0).is_err());
    assert!(chan.set_channel(89).is_ok());
    assert!(chan.set_channel(128).is_err());
    assert!(chan.set_channel(200).is_err());
}

#[test]
fn baud_rate_set() {
    let mut params = Parameters::default();
    params.set_baud_rate(BaudRate::Bps115200).unwrap();
    assert_eq!(params.baud_rate, BaudRate::Bps115200);
    params.mode = Mode::Fu1;
    params.set_baud_rate(BaudRate::Bps1200).unwrap();
    assert_eq!(params.baud_rate, BaudRate::Bps1200);
    assert_eq!(params.get_air_baud_rate(), AirBaudRate::Bps250000);
}

#[test]
fn baud_rate_to_primitive() {
    assert_eq!(1200, BaudRate::Bps1200.to_u32().unwrap());
    assert_eq!(2400, BaudRate::Bps2400.to_u32().unwrap());
    assert_eq!(4800, BaudRate::Bps4800.to_u32().unwrap());
    assert_eq!(9600, BaudRate::Bps9600.to_u32().unwrap());
    assert_eq!(19200, BaudRate::Bps19200.to_u32().unwrap());
    assert_eq!(38400, BaudRate::Bps38400.to_u32().unwrap());
    assert_eq!(57600, BaudRate::Bps57600.to_u32().unwrap());
    assert_eq!(115200, BaudRate::Bps115200.to_u32().unwrap());
}

#[test]
fn baud_rate_from_primitive() {
    assert_eq!(BaudRate::Bps1200, BaudRate::from_u32(1200).unwrap());
    assert_eq!(BaudRate::Bps2400, BaudRate::from_u32(2400).unwrap());
    assert_eq!(BaudRate::Bps4800, BaudRate::from_u32(4800).unwrap());
    assert_eq!(BaudRate::Bps9600, BaudRate::from_u32(9600).unwrap());
    assert_eq!(BaudRate::Bps19200, BaudRate::from_u32(19200).unwrap());
    assert_eq!(BaudRate::Bps38400, BaudRate::from_u32(38400).unwrap());
    assert_eq!(BaudRate::Bps57600, BaudRate::from_u32(57600).unwrap());
    assert_eq!(BaudRate::Bps115200, BaudRate::from_u32(115200).unwrap());
}
