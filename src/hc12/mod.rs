use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::{
    digital::v2::OutputPin,
    serial::{Read, Write},
};
use nb::*;

use crate::config::{Fu3, Parameters};

#[derive(Debug)]
pub struct Hc12<S, P, D, M>
where
    S: Read<u8> + Write<u8>,
    P: OutputPin,
    D: DelayMs<u32>,
{
    serial: S,
    set_pin: P,
    delay: D,
    parameters: Parameters<M>,
}

impl<S, P, D> Hc12<S, P, D, Fu3>
where
    S: Read<u8> + Write<u8>,
    P: OutputPin,
    D: DelayMs<u32>,
{
    pub fn new(serial: S, set_pin: P, delay: D) -> Self {
        Self {
            serial,
            set_pin,
            delay,
            parameters: Parameters::default(),
        }
    }

    pub fn release(mut self) -> (S, P, D) {
        self.set_pin.set_high().ok().unwrap();
        self.delay.delay_ms(12);
        (self.serial, self.set_pin, self.delay)
    }

    pub fn write_buffer(&mut self, buffer: &[u8]) -> Result<(), Error<crate::Error>> {
        for ch in buffer.iter() {
            match block!(self.serial.write(*ch)) {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        Ok(())
    }

    pub fn read_buffer(&mut self, buffer: &mut [u8]) -> Result<(), Error<crate::Error>> {
        let mut n = 0;
        while n < buffer.len() {
            if let Ok(ch) = block!(self.serial.read()) {
                buffer[n] = ch;
                n += 1;
            }
        }
        Ok(())
    }
}

impl<S, P, D, T> embedded_hal::serial::Read<u8> for Hc12<S, P, D, T>
where
    S: Read<u8> + Write<u8>,
    P: OutputPin,
    D: DelayMs<u32>,
{
    type Error = Self::Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.serial.read()
    }
}

impl<S, P, D, T> embedded_hal::serial::Write<u8> for Hc12<S, P, D, T>
where
    S: embedded_hal::serial::Read<u8> + embedded_hal::serial::Write<u8>,
    P: OutputPin,
    D: DelayMs<u32>,
{
    type Error = nb::Error<u8>;
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.serial.write(word)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.serial.flush()
    }
}
