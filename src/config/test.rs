use super::*;

use num_traits::{FromPrimitive, ToPrimitive};

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
    assert_eq!(Channel::try_from(response).unwrap(), channel);
    let response = b"OK+RC100\r\n";
    let channel = Channel(100);
    assert_eq!(Channel::try_from(response).unwrap(), channel);
    let response = b"OK+RC099\r\n";
    let channel = Channel(99);
    assert_eq!(Channel::try_from(response).unwrap(), channel);
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
