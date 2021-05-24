//! Type-state driven state machine of Hc12.

use core::convert::TryFrom;
use core::marker::PhantomData;

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::{
    digital::v2::OutputPin,
    serial::{Read, Write},
};
use nb::*;

use crate::settings::parameter::{
    baudrate::BaudRate, channel::Channel, mode::Mode, parameters::Parameters,
    transmission_power::TransmissionPower, OK_QUERY, OK_RESPONSE, QUERY_PARAMS_COMMAND,
    RESET_SETTINGS_COMMAND, RESET_SETTINGS_RESPONSE, SLEEP_COMMAND, SLEEP_RESPONSE, VERSION_QUERY,
};

#[cfg(test)]
mod test;

/// Normal mode marker
pub struct Normal;

/// Configuration mode marker
pub struct Configuration;

/// Sleep mode marker
pub struct Sleep;

/// Fallible transition from normal to config state
type NormalToConfig<S, P, D> =
    core::result::Result<Hc12<S, P, D, Configuration>, Hc12<S, P, D, Normal>>;

/// Fallible transition from config to normal state
type ConfigToNormal<S, P, D> =
    core::result::Result<Hc12<S, P, D, Normal>, Hc12<S, P, D, Configuration>>;

/// Fallible transition from config to sleep state
type ConfigToSleep<S, P, D> =
    core::result::Result<Hc12<S, P, D, Sleep>, Hc12<S, P, D, Configuration>>;

/// Fallible transition from sleep to config state
type SleepToConfig<S, P, D> =
    core::result::Result<Hc12<S, P, D, Configuration>, Hc12<S, P, D, Sleep>>;

/// Hc12 resources: A serial port, an output pin, and a Delay.
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
    mode: PhantomData<M>,
}

/// Implementation for normal mode of Hc12
impl<S, P, D> Hc12<S, P, D, Normal>
where
    S: Read<u8> + Write<u8>,
    P: OutputPin,
    D: DelayMs<u32>,
{
    /// Construct an Hc12 in normal mode
    pub fn new(serial: S, mut set_pin: P, mut delay: D) -> Self {
        let _ = set_pin.set_high();
        delay.delay_ms(20); // TODO which duration?
                            // TODO read configuration; if it does not work, baud rate is wrong.
                            // TODO when e-hal supports changing the baud rate, try to probe the right one
        Self {
            serial,
            set_pin,
            delay,
            mode: PhantomData::<Normal>,
        }
    }

    /// Release the contained resources
    pub fn release(self) -> (S, P, D) {
        (self.serial, self.set_pin, self.delay)
    }

    /// Write entire buffer to serial port
    pub fn write_buffer(&mut self, buffer: &[u8]) -> Result<(), Error<crate::Error>> {
        for ch in buffer {
            let _ = block!(self.serial.write(*ch));
        }
        Ok(())
    }

    /// Read entire buffer from serial port
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

    /// Move Hc12 in normal mode to Hc12 in config mode
    pub fn into_configuration_mode(mut self) -> NormalToConfig<S, P, D> {
        let _ = self.set_pin.set_low();
        self.delay.delay_ms(40); // TODO how long?
        Ok(Hc12 {
            serial: self.serial,
            set_pin: self.set_pin,
            delay: self.delay,
            mode: PhantomData::<Configuration>,
        })
    }
}

/// Implement Read for Hc12 in normal mode
/// This just defers to the underlying serial implementation.
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

/// Implement Write for Hc12 in normal mode.
/// This just defers to the underlying serial implementation.
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

/// Implementation for Hc12 in config mode.
impl<S, P, D> Hc12<S, P, D, Configuration>
where
    S: Read<u8> + Write<u8>,
    P: OutputPin,
    D: DelayMs<u32>,
{
    /// Move Hc12 in config mode back to normal mode.
    pub fn into_normal_mode(mut self) -> ConfigToNormal<S, P, D> {
        let _ = self.set_pin.set_high();
        self.delay.delay_ms(40); // TODO how long?
        Ok(Hc12 {
            serial: self.serial,
            set_pin: self.set_pin,
            delay: self.delay,
            mode: PhantomData::<Normal>,
        })
    }

    /// Move Hc12 in config mode into sleeping mode.
    pub fn into_sleeping_mode(mut self) -> ConfigToSleep<S, P, D> {
        for ch in &SLEEP_COMMAND {
            let _ = block!(self.serial.write(*ch));
        }
        let mut n = 0;
        let mut response = [0u8; 10];
        for i in &mut response {
            if let Ok(ch) = block!(self.serial.read()) {
                *i = ch;
                n += 1;
                if ch == b'\n' {
                    break;
                }
            }
        }
        if n == SLEEP_RESPONSE.len() && response[..n] == SLEEP_RESPONSE[..n] {
            let _ = self.set_pin.set_high();
            Ok(Hc12 {
                delay: self.delay,
                mode: PhantomData::<Sleep>,
                set_pin: self.set_pin,
                serial: self.serial,
            })
        } else {
            Err(self)
        }
    }

    /// Check if Hc12 responds to "AT" query with "OK".
    pub fn is_ok(&mut self) -> bool {
        for ch in &OK_QUERY {
            let _ = block!(self.serial.write(*ch));
        }
        let mut n = 0;
        let mut buffer = [0u8; 4];
        while n < 4 {
            if let Ok(ch) = block!(self.serial.read()) {
                buffer[n] = ch;
                n += 1;
            }
        }
        buffer == OK_RESPONSE
    }

    /// Get version as a string from Hc12
    pub fn get_version<'a>(&mut self, buffer: &'a mut [u8; 64]) -> &'a [u8] {
        for ch in &VERSION_QUERY {
            let _ = block!(self.serial.write(*ch));
        }
        let mut count = 0;
        for v in buffer.iter_mut() {
            if let Ok(ch) = block!(self.serial.read()) {
                *v = ch;
                count += 1;
                if ch == b'\n' {
                    break;
                }
            }
        }
        &buffer[..count]
    }

    /// Reset Hc12 to default settings.
    pub fn reset_settings(&mut self) -> bool {
        for ch in &RESET_SETTINGS_COMMAND {
            let _ = block!(self.serial.write(*ch));
        }
        let mut response = [0u8; 12];
        let mut count = 0;
        for v in &mut response {
            if let Ok(ch) = block!(self.serial.read()) {
                *v = ch;
                count += 1;
                if ch == b'\n' {
                    break;
                }
            }
        }
        count == RESET_SETTINGS_RESPONSE.len()
            && response[..count] == RESET_SETTINGS_RESPONSE[..count]
    }

    /// Get parameters of Hc12
    pub fn get_parameters(&mut self) -> Option<Parameters> {
        for ch in &QUERY_PARAMS_COMMAND {
            let _ = block!(self.serial.write(*ch));
        }
        let mut params = [[0u8; 16]; 4];
        let mut param_slices: [&[u8]; 4] = Default::default();
        for (pi, p) in &mut params.iter_mut().enumerate() {
            for (i, v) in p.iter_mut().enumerate() {
                if let Ok(ch) = block!(self.serial.read()) {
                    *v = ch;
                    if ch == b'\n' {
                        param_slices[pi] = &p[..=i];
                        break;
                    }
                }
            }
        }
        let mode = Mode::try_from(param_slices[0]).ok()?;
        let baud_rate = BaudRate::try_from(param_slices[1]).ok()?;
        let channel = Channel::try_from(param_slices[2]).ok()?;
        let power = TransmissionPower::try_from(param_slices[3]).ok()?;
        Some(Parameters {
            baud_rate,
            channel,
            power,
            mode,
        })
    }
}

/// Implementation for Hc12 in sleeping mode.
impl<S, P, D> Hc12<S, P, D, Sleep>
where
    S: Read<u8> + Write<u8>,
    P: OutputPin,
    D: DelayMs<u32>,
{
    /// Move Hc12 in sleeping mode back to configuration mode.
    pub fn into_configuration_mode(mut self) -> SleepToConfig<S, P, D> {
        let _ = self.set_pin.set_low();
        self.delay.delay_ms(40); // TODO how long?
        Ok(Hc12 {
            serial: self.serial,
            set_pin: self.set_pin,
            delay: self.delay,
            mode: PhantomData::<Configuration>,
        })
    }
}
