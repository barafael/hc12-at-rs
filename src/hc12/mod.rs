use core::marker::PhantomData;

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::{
    digital::v2::OutputPin,
    serial::{Read, Write},
};
use nb::*;

use crate::config::Parameters;

/// Normal mode marker
pub struct Normal;

/// Configuration mode marker
pub struct Configuration;

#[derive(Debug)]
pub struct Hc12<S, P, D, T>
where
    S: Read<u8> + Write<u8>,
    P: OutputPin,
    D: DelayMs<u32>,
{
    serial: S,
    set_pin: P,
    delay: D,
    parameters: Parameters,
    mode: PhantomData<T>,
}

impl<S, P, D> Hc12<S, P, D, Normal>
where
    S: Read<u8> + Write<u8>,
    P: OutputPin,
    D: DelayMs<u32>,
{
    pub fn new(serial: S, mut set_pin: P, mut delay: D) -> Self {
        set_pin.set_high();
        delay.delay_ms(20); // TODO which duration?
                            // TODO read configuration; if it does not work, baud rate is wrong.
                            // TODO when e-hal supports changing the baud rate, try to probe the right one
        Self {
            serial,
            set_pin,
            delay,
            parameters: Parameters::default(),
            mode: PhantomData::<Normal>,
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

    pub fn into_configuration_mode(mut self) -> Hc12<S, P, D, Configuration> {
        let _ = self.set_pin.set_low();
        self.delay.delay_ms(40); // TODO how long?
        Hc12 {
            serial: self.serial,
            set_pin: self.set_pin,
            parameters: self.parameters,
            delay: self.delay,
            mode: PhantomData::<Configuration>,
        }
    }
}

impl<S, P, D> embedded_hal::serial::Read<u8> for Hc12<S, P, D, Normal>
where
    S: Read<u8> + Write<u8>,
    P: OutputPin,
    D: DelayMs<u32>,
{
    type Error = <S as Read<u8>>::Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.serial.read()
    }
}

impl<S, P, D> embedded_hal::serial::Write<u8> for Hc12<S, P, D, Normal>
where
    S: embedded_hal::serial::Read<u8> + embedded_hal::serial::Write<u8>,
    P: OutputPin,
    D: DelayMs<u32>,
{
    type Error = <S as Write<u8>>::Error;
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.serial.write(word)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.serial.flush()
    }
}

impl<S, P, D> Hc12<S, P, D, Configuration>
where
    S: Read<u8> + Write<u8>,
    P: OutputPin,
    D: DelayMs<u32>,
{
    pub fn into_normal_mode(mut self) -> Hc12<S, P, D, Normal> {
        let _ = self.set_pin.set_high();
        self.delay.delay_ms(40); // TODO how long?
        Hc12 {
            serial: self.serial,
            set_pin: self.set_pin,
            parameters: self.parameters,
            delay: self.delay,
            mode: PhantomData::<Normal>,
        }
    }

    pub fn is_ok(&mut self) -> bool {
        for ch in b"AT\r\n".iter() {
            match block!(self.serial.write(*ch)) {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        let mut n = 0;
        let mut buffer = [0u8; 4];
        while n < 4 {
            if let Ok(ch) = block!(self.serial.read()) {
                buffer[n] = ch;
                n += 1;
            }
        }
        return buffer == *b"OK\r\n";
    }
}
