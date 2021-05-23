use crate::parameter::transmission_power::TransmissionPower;

use super::*;

use debugless_unwrap::DebuglessUnwrap;
use embedded_hal_mock::pin::*;
use embedded_hal_mock::{delay::MockNoop, pin, serial};

#[test]
fn is_ok() {
    let delay = MockNoop;
    let pin_transactions = [
        pin::Transaction::set(State::High),
        pin::Transaction::set(State::Low),
        pin::Transaction::set(State::High),
    ];
    let set_pin = pin::Mock::new(&pin_transactions);
    let transactions = [
        serial::Transaction::write_many(b"AT\r\n"),
        serial::Transaction::read_many(b"OK\r\n"),
    ];
    let serial = serial::Mock::new(&transactions);
    let hc12 = Hc12::new(serial, set_pin, delay);
    let mut hc12 = hc12.into_configuration_mode().debugless_unwrap();
    let ok = hc12.is_ok();
    assert!(ok);
    let hc12 = hc12.into_normal_mode().debugless_unwrap();
    let (mut serial, mut set_pin, _) = hc12.release();
    serial.done();
    set_pin.done();
}

#[test]
fn send_buffer() {
    let delay = MockNoop;
    let set_pin = pin::Mock::new(&[pin::Transaction::set(State::High)]);
    let transactions = [serial::Transaction::write_many(b"some data AT AT\r\n")];
    let serial = serial::Mock::new(&transactions);
    let mut hc12 = Hc12::new(serial, set_pin, delay);
    hc12.write_buffer(b"some data AT AT\r\n").unwrap();
    let (mut serial, mut set_pin, _) = hc12.release();
    serial.done();
    set_pin.done();
}

#[test]
fn receive_buffer() {
    let delay = MockNoop;
    let set_pin = pin::Mock::new(&[pin::Transaction::set(State::High)]);
    let transactions = [serial::Transaction::read_many(b"some data AT AT\r\n")];
    let serial = serial::Mock::new(&transactions);
    let mut hc12 = Hc12::new(serial, set_pin, delay);
    let mut buffer = [0u8; 32];
    hc12.read_buffer(&mut buffer[..17]).unwrap();
    assert_eq!(&buffer[..17], &b"some data AT AT\r\n"[..]);
    let (mut serial, mut set_pin, _) = hc12.release();
    serial.done();
    set_pin.done();
}

#[test]
fn usage_from_readme() {
    let delay = MockNoop;
    let set_pin = pin::Mock::new(&[
        pin::Transaction::set(State::High),
        pin::Transaction::set(State::Low),
        pin::Transaction::set(State::High),
    ]);
    let transactions = [
        serial::Transaction::write_many(b"AT\r\n"),
        serial::Transaction::read_many(b"OK\r\n"),
        serial::Transaction::write_many(b"AT+V\r\n"),
        serial::Transaction::read_many(b"VERSION-42\r\n"),
        serial::Transaction::write_many(b"AT+RX\r\n"),
        serial::Transaction::read_many(b"OK+FU2\r\nOK+B115200\r\nOK+RC042\r\nOK+RP:-1dBm\r\n"),
        serial::Transaction::write_many(b"some data AT AT\r\n"),
    ];
    let serial = serial::Mock::new(&transactions);
    let hc12 = Hc12::new(serial, set_pin, delay);
    let mut hc12 = hc12.into_configuration_mode().debugless_unwrap();

    let mut buffer = [0u8; 16];

    let ok = hc12.is_ok();
    assert!(ok);

    let version = hc12.get_version(&mut buffer);
    assert_eq!(version, b"VERSION-42\r\n");

    let params = hc12.get_parameters().unwrap();
    assert_eq!(
        Parameters {
            baud_rate: BaudRate::Bps115200,
            channel: Channel::new(42).unwrap(),
            power: TransmissionPower(1),
            mode: Mode::Fu2,
        },
        params
    );

    let mut hc12 = hc12.into_normal_mode().debugless_unwrap();

    hc12.write_buffer(b"some data AT AT\r\n").unwrap();

    let (mut serial, mut set_pin, _) = hc12.release();
    serial.done();
    set_pin.done();
}

#[test]
fn get_version() {
    let delay = MockNoop;
    let pin_transactions = [
        pin::Transaction::set(State::High),
        pin::Transaction::set(State::Low),
        pin::Transaction::set(State::High),
    ];
    let set_pin = pin::Mock::new(&pin_transactions);
    let transactions = [
        serial::Transaction::write_many(b"AT+V\r\n"),
        serial::Transaction::read_many(b"HC-12_VFAKE\r\n"),
    ];
    let serial = serial::Mock::new(&transactions);
    let hc12 = Hc12::new(serial, set_pin, delay);
    let mut hc12 = hc12.into_configuration_mode().debugless_unwrap();
    let mut buffer = [0u8; 16];
    let result = hc12.get_version(&mut buffer);
    assert_eq!(result, b"HC-12_VFAKE\r\n");
    let hc12 = hc12.into_normal_mode().debugless_unwrap();
    let (mut serial, mut set_pin, _) = hc12.release();
    serial.done();
    set_pin.done();
}

#[test]
fn get_parameters() {
    let delay = MockNoop;
    let pin_transactions = [
        pin::Transaction::set(State::High),
        pin::Transaction::set(State::Low),
        pin::Transaction::set(State::High),
    ];
    let set_pin = pin::Mock::new(&pin_transactions);
    let transactions = [
        serial::Transaction::write_many(b"AT+RX\r\n"),
        serial::Transaction::read_many(b"OK+FU3\r\nOK+B9600\r\nOK+RC001\r\nOK+RP:+20dBm\r\n"),
    ];
    let serial = serial::Mock::new(&transactions);
    let hc12 = Hc12::new(serial, set_pin, delay);
    let mut hc12 = hc12.into_configuration_mode().debugless_unwrap();
    let params = hc12.get_parameters().unwrap();
    let hc12 = hc12.into_normal_mode().debugless_unwrap();
    let (mut serial, mut set_pin, _) = hc12.release();
    serial.done();
    set_pin.done();
    let expected = Parameters {
        baud_rate: BaudRate::Bps9600,
        channel: Channel::new(1).unwrap(),
        power: TransmissionPower(8),
        mode: Mode::Fu3,
    };
    assert_eq!(expected, params);
}

#[test]
fn get_more_parameters() {
    let delay = MockNoop;
    let pin_transactions = [
        pin::Transaction::set(State::High),
        pin::Transaction::set(State::Low),
        pin::Transaction::set(State::High),
    ];
    let set_pin = pin::Mock::new(&pin_transactions);
    let transactions = [
        serial::Transaction::write_many(b"AT+RX\r\n"),
        serial::Transaction::read_many(b"OK+FU1\r\nOK+B115200\r\nOK+RC101\r\nOK+RP:-1dBm\r\n"),
    ];
    let serial = serial::Mock::new(&transactions);
    let hc12 = Hc12::new(serial, set_pin, delay);
    let mut hc12 = hc12.into_configuration_mode().debugless_unwrap();
    let params = hc12.get_parameters().unwrap();
    let hc12 = hc12.into_normal_mode().debugless_unwrap();
    let (mut serial, mut set_pin, _) = hc12.release();
    serial.done();
    set_pin.done();
    let expected = Parameters {
        baud_rate: BaudRate::Bps115200,
        channel: Channel::new(101).unwrap(),
        power: TransmissionPower(1),
        mode: Mode::Fu1,
    };
    assert_eq!(expected, params);
}

#[test]
fn reset_to_default() {
    let delay = MockNoop;
    let pin_transactions = [
        pin::Transaction::set(State::High),
        pin::Transaction::set(State::Low),
        pin::Transaction::set(State::High),
    ];
    let set_pin = pin::Mock::new(&pin_transactions);
    let transactions = [
        serial::Transaction::write_many(b"AT+DEFAULT\r\n"),
        serial::Transaction::read_many(b"OK+DEFAULT\r\n"),
    ];
    let serial = serial::Mock::new(&transactions);
    let hc12 = Hc12::new(serial, set_pin, delay);
    let mut hc12 = hc12.into_configuration_mode().debugless_unwrap();
    let result = hc12.reset_settings();
    assert!(result);
    let hc12 = hc12.into_normal_mode().debugless_unwrap();
    let (mut serial, mut set_pin, _) = hc12.release();
    serial.done();
    set_pin.done();
}

#[test]
fn go_to_sleep_and_wake_up() {
    let delay = MockNoop;
    let pin_transactions = [
        pin::Transaction::set(State::High),
        pin::Transaction::set(State::Low),
        pin::Transaction::set(State::High),
        pin::Transaction::set(State::Low),
        pin::Transaction::set(State::High),
    ];
    let set_pin = pin::Mock::new(&pin_transactions);
    let transactions = [
        serial::Transaction::write_many(b"AT+SLEEP\r\n"),
        serial::Transaction::read_many(b"OK+SLEEP\r\n"),
    ];
    let serial = serial::Mock::new(&transactions);
    let hc12 = Hc12::new(serial, set_pin, delay);
    let hc12 = hc12.into_configuration_mode().debugless_unwrap();
    let result = hc12.into_sleeping_mode();
    let hc12 = result.debugless_unwrap();
    let result = hc12.into_configuration_mode();
    let hc12 = result.debugless_unwrap();
    let hc12 = hc12.into_normal_mode().debugless_unwrap();
    let (mut serial, mut set_pin, _) = hc12.release();
    serial.done();
    set_pin.done();
}
