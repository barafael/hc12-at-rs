use super::*;

use debugless_unwrap::DebuglessUnwrap;
use embedded_hal_mock::delay::MockNoop;
use embedded_hal_mock::pin::*;

#[test]
fn is_ok() {
    let delay = MockNoop;
    let pin_transactions = [
        embedded_hal_mock::pin::Transaction::set(State::High),
        embedded_hal_mock::pin::Transaction::set(State::Low),
        embedded_hal_mock::pin::Transaction::set(State::High),
    ];
    let set_pin = embedded_hal_mock::pin::Mock::new(&pin_transactions);
    let transactions = [
        embedded_hal_mock::serial::Transaction::write_many(b"AT\r\n"),
        embedded_hal_mock::serial::Transaction::read_many(b"OK\r\n"),
    ];
    let serial = embedded_hal_mock::serial::Mock::new(&transactions);
    let hc12 = Hc12::new(serial, set_pin, delay);
    let mut hc12 = hc12.into_configuration_mode();
    let ok = hc12.is_ok();
    assert!(ok);
    let hc12 = hc12.into_normal_mode();
    let (mut serial, mut set_pin, _) = hc12.release();
    serial.done();
    set_pin.done();
}

#[test]
fn get_version() {
    let delay = MockNoop;
    let pin_transactions = [
        embedded_hal_mock::pin::Transaction::set(State::High),
        embedded_hal_mock::pin::Transaction::set(State::Low),
        embedded_hal_mock::pin::Transaction::set(State::High),
    ];
    let set_pin = embedded_hal_mock::pin::Mock::new(&pin_transactions);
    let transactions = [
        embedded_hal_mock::serial::Transaction::write_many(b"AT+V\r\n"),
        embedded_hal_mock::serial::Transaction::read_many(b"HC-12_VFAKE\r\n"),
    ];
    let serial = embedded_hal_mock::serial::Mock::new(&transactions);
    let hc12 = Hc12::new(serial, set_pin, delay);
    let mut hc12 = hc12.into_configuration_mode();
    let mut buffer = [0u8; 16];
    let result = hc12.get_version(&mut buffer);
    assert_eq!(result, b"HC-12_VFAKE\r\n");
    let hc12 = hc12.into_normal_mode();
    let (mut serial, mut set_pin, _) = hc12.release();
    serial.done();
    set_pin.done();
}

#[test]
fn reset_to_default() {
    let delay = MockNoop;
    let pin_transactions = [
        embedded_hal_mock::pin::Transaction::set(State::High),
        embedded_hal_mock::pin::Transaction::set(State::Low),
        embedded_hal_mock::pin::Transaction::set(State::High),
    ];
    let set_pin = embedded_hal_mock::pin::Mock::new(&pin_transactions);
    let transactions = [
        embedded_hal_mock::serial::Transaction::write_many(b"AT+DEFAULT\r\n"),
        embedded_hal_mock::serial::Transaction::read_many(b"OK+DEFAULT\r\n"),
    ];
    let serial = embedded_hal_mock::serial::Mock::new(&transactions);
    let hc12 = Hc12::new(serial, set_pin, delay);
    let mut hc12 = hc12.into_configuration_mode();
    let result = hc12.reset_settings();
    assert!(result);
    let hc12 = hc12.into_normal_mode();
    let (mut serial, mut set_pin, _) = hc12.release();
    serial.done();
    set_pin.done();
}

#[test]
fn go_to_sleep_and_wake_up() {
    let delay = MockNoop;
    let pin_transactions = [
        embedded_hal_mock::pin::Transaction::set(State::High),
        embedded_hal_mock::pin::Transaction::set(State::Low),
        embedded_hal_mock::pin::Transaction::set(State::High),
        embedded_hal_mock::pin::Transaction::set(State::Low),
        embedded_hal_mock::pin::Transaction::set(State::High),
    ];
    let set_pin = embedded_hal_mock::pin::Mock::new(&pin_transactions);
    let transactions = [
        embedded_hal_mock::serial::Transaction::write_many(b"AT+SLEEP\r\n"),
        embedded_hal_mock::serial::Transaction::read_many(b"OK+SLEEP\r\n"),
    ];
    let serial = embedded_hal_mock::serial::Mock::new(&transactions);
    let hc12 = Hc12::new(serial, set_pin, delay);
    let hc12 = hc12.into_configuration_mode();
    let result = hc12.into_sleeping_mode();
    let hc12 = result.debugless_unwrap();
    let result = hc12.into_configuration_mode();
    let hc12 = result.debugless_unwrap();
    let hc12 = hc12.into_normal_mode();
    let (mut serial, mut set_pin, _) = hc12.release();
    serial.done();
    set_pin.done();
}
