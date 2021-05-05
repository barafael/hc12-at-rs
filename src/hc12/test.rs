use super::*;

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
