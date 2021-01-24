use core::marker::PhantomData;

use embedded_hal::{blocking::delay, digital::v2::OutputPin};
use embedded_hal::serial::*;
use at_commands::builder::CommandBuilder;

use crate::delay::DelayMs;
use crate::delay::DelayUs;

pub struct Normal;

pub struct Command;

#[derive(Debug, Default)]
struct Hc12<P, S, D, T> {
    set_pin: P,
    delay: D,
    serial: S,
    state: PhantomData<T>,
}

impl<P, S, D> Hc12<P, S, D, Normal>
where P: OutputPin, D: DelayUs<u16> + DelayMs<u16>, S: Read<char> + Write<char> {
    pub fn new(mut set_pin: P, mut delay: D, mut serial: S) -> Option<Self> {
        set_pin.set_low().ok()?;
        delay.delay_ms(50);
        serial.write('A').ok()?;
        serial.write('T').ok()?;
        serial.write('\r').ok()?;
        serial.write('\n').ok()?;
        let mut n = 0;
        let answer = "Ok\r\n";
        while n < 4 {
            if let Ok(ch) = serial.read() {
                if ch != answer.chars().nth(n).unwrap() {
                    set_pin.set_high().ok()?;
                    return None;
                }
                n += 1;
            }
        }
        set_pin.set_high().ok()?;
        Some(Hc12 {
            set_pin,
            delay,
            serial,
            state: PhantomData::<Normal>,
        })
    }

    pub fn release(self) -> (P, D, S) {
        (self.set_pin, self.delay, self.serial)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::{pin::{self, State}, serial};

    #[test]
    fn bare_at_some() {
        let expectations = [
            serial::Transaction::write('A'),
            serial::Transaction::write('T'),
            serial::Transaction::write('\r'),
            serial::Transaction::write('\n'),
            serial::Transaction::read('O'),
            serial::Transaction::read('k'),
            serial::Transaction::read('\r'),
            serial::Transaction::read('\n'),
        ];
        let pin_expectations = [
            pin::Transaction::set(State::Low),
            pin::Transaction::set(State::High),
        ];
        let serial = serial::Mock::new(&expectations);
        let delay = embedded_hal_mock::delay::MockNoop;
        let set_pin = embedded_hal_mock::pin::Mock::new(&pin_expectations);
        let hc12 = Hc12::new(set_pin, delay, serial).unwrap();
        let (mut p, _, mut s) = hc12.release();
        p.done();
        s.done();
    }

    #[test]
    fn bare_at_none() {
        let expectations = [
            serial::Transaction::write('A'),
            serial::Transaction::write('T'),
            serial::Transaction::write('\r'),
            serial::Transaction::write('\n'),
            serial::Transaction::read('B'),
            serial::Transaction::read('l'),
            serial::Transaction::read('a'),
            serial::Transaction::read('\r'),
            serial::Transaction::read('\n'),
        ];
        let pin_expectations = [
            pin::Transaction::set(State::Low),
            pin::Transaction::set(State::High),
        ];
        let serial = serial::Mock::new(&expectations);
        let delay = embedded_hal_mock::delay::MockNoop;
        let set_pin = embedded_hal_mock::pin::Mock::new(&pin_expectations);
        let _ = Hc12::new(set_pin, delay, serial).is_none();
    }
}
