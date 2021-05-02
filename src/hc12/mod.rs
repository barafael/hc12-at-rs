use crate::config;

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::prelude::_embedded_hal_serial_Write;
use embedded_hal::{
    digital::v2::OutputPin,
    serial::{Read, Write},
};
use nb::*;

#[derive(Debug)]
pub struct Hc12<S, D, O>
where
    D: DelayMs<u32>,
    S: Read<u8> + Write<u8>,
    O: OutputPin,
{
    delay: D,
    serial: S,
    set_pin: O,
}

impl<S, D, O> Hc12<S, D, O>
where
    D: DelayMs<u32>,
    S: Read<u8> + Write<u8>,
    O: OutputPin,
{
    pub fn new(delay: D, serial: S, set_pin: O) -> Self {
        Self {
            delay,
            serial,
            set_pin,
        }
    }

    pub fn release(mut self) -> (S, D, O) {
        self.set_pin.set_high().ok().unwrap();
        self.delay.delay_ms(12);
        (self.serial, self.delay, self.set_pin)
    }
    pub fn write_buffer(&mut self, buffer: &[u8]) -> Result<(), Error<crate::Error>> {
        for ch in buffer.iter() {
            match self.serial.write(*ch) {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        Ok(())
    }

    pub fn read_buffer(&mut self, buffer: &mut [u8]) -> Result<(), Error<crate::Error>> {
        let mut n = 0;
        while n < buffer.len() {
            if let Ok(ch) = self.serial.read() {
                buffer[n] = ch;
                n += 1;
            }
        }
        Ok(())
    }
}
