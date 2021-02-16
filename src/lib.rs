#![no_std]

use core::marker::PhantomData;

use at_commands::builder::CommandBuilder;
use embedded_hal::serial::*;
use embedded_hal::{blocking::delay, digital::v2::OutputPin};

use crate::delay::DelayMs;

enum Mode {
    Fu1,
    Fu2,
    Fu3,
    Fu4,
}

pub struct NormalState {
    mode: Mode,
}

pub struct CommandState;

pub struct SleepState;

pub enum Error {
    Read,
    Write,
    InvalidChannel,
}

pub enum ChannelError {
    InvalidChannel,
}

impl From<ChannelError> for Error {
    fn from(v: ChannelError) -> Self {
        match v {
            ChannelError::InvalidChannel => Error::InvalidChannel,
        }
    }
}

pub enum BaudRate {
    Bps1200,
    Bps2400,
    Bps4800,
    Bps9600,
    Bps19200,
    Bps38400,
    Bps57600,
    Bps115200,
}

impl Default for BaudRate {
    fn default() -> Self {
        BaudRate::Bps9600
    }
}

pub struct Channel {
    channel: u8,
}

impl Default for Channel {
    fn default() -> Self {
        Channel { channel: 1 }
    }
}

impl Channel {
    fn get_freq_mhz(&self) -> f32 {
        433.0 + self.channel as f32 * 0.4
    }

    fn set_channel(&mut self, ch: u8) -> Result<(), ChannelError> {
        if ch != 0 && ch < 128 {
            self.channel = ch;
            Ok(())
        } else {
            Err(ChannelError::InvalidChannel)
        }
    }
}

pub struct TransmissionPower {
    power: u8,
}

impl Default for TransmissionPower {
    fn default() -> Self {
        TransmissionPower { power: 8 }
    }
}

impl TransmissionPower {
    fn get_power_dbm(&self) -> i8 {
        match self.power {
            1 => -1,
            2 => 2,
            3 => 5,
            4 => 8,
            5 => 11,
            6 => 14,
            7 => 17,
            8 => 20,
            _ => unreachable!(),
        }
    }

    fn get_power_milli_watt(&self) -> f32 {
        match self.power {
            1 => 0.79,
            2 => 1.58,
            3 => 3.16,
            4 => 6.31,
            5 => 12.59,
            6 => 25.12,
            7 => 50.12,
            8 => 100.0,
            _ => unreachable!(),
        }
    }
}

struct Parameters {
    mode: PhantomData<Mode>,
    baud_rate: BaudRate,
    channel: Channel,
    power: TransmissionPower,
}

#[derive(Debug, Default)]
struct Hc12<P, S, D, T> {
    set_pin: P,
    delay: D,
    serial: S,
    state: PhantomData<T>,
}

impl<P, S, D> Hc12<P, S, D, NormalState>
where
    P: OutputPin,
    D: DelayMs<u16>,
    S: Read<u8> + Write<u8>,
{
    fn write_buffer(serial: &mut S, buffer: &[u8]) -> Result<(), Error> {
        for ch in buffer.iter() {
            match serial.write(*ch) {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        Ok(())
    }

    fn read_buffer(serial: &mut S, buffer: &mut [u8]) -> Result<(), Error> {
        let mut n = 0;
        while n < buffer.len() {
            if let Ok(ch) = serial.read() {
                buffer[n] = ch;
                n += 1;
            }
        }
        Ok(())
    }

    pub fn new(mut set_pin: P, mut delay: D, mut serial: S) -> Option<Self> {
        set_pin.set_low().ok()?;
        delay.delay_ms(50);
        let mut buffer: [u8; 4] = [0; 4];
        let command = CommandBuilder::create_execute(&mut buffer, true)
            .named("")
            .finish()
            .unwrap();
        if Hc12::<P, S, D, NormalState>::write_buffer(&mut serial, command).is_err() {
            set_pin.set_high().ok()?;
            return None;
        }
        let mut buffer: [u8; 4] = [0; 4];
        if Hc12::<P, S, D, NormalState>::read_buffer(&mut serial, &mut buffer).is_err() {
            set_pin.set_high().ok()?;
            return None;
        }
        set_pin.set_high().ok()?;
        if buffer != ['O' as u8, 'k' as u8, '\r' as u8, '\n' as u8] {
            return None;
        }
        Some(Hc12 {
            set_pin,
            delay,
            serial,
            state: PhantomData::<NormalState>,
        })
    }

    pub fn release(self) -> (P, D, S) {
        (self.set_pin, self.delay, self.serial)
    }

    pub fn go_to_command_state(mut self) -> Option<Hc12<P, S, D, CommandState>> {
        self.set_pin.set_low().ok()?;
        self.delay.delay_ms(50);
        Some(Hc12 {
            set_pin: self.set_pin,
            serial: self.serial,
            delay: self.delay,
            state: PhantomData::<CommandState>,
        })
    }
}

impl<P, S, D> Hc12<P, S, D, CommandState>
where
    P: OutputPin,
    D: DelayMs<u16>,
    S: Read<u8> + Write<u8>,
{
    pub fn get_firmware_version(&mut self) -> &str {
        "TODO"
    }

    pub fn go_to_sleep(self) -> Hc12<P, S, D, SleepState> {
        todo!();
        Hc12 {
            set_pin: self.set_pin,
            serial: self.serial,
            delay: self.delay,
            state: PhantomData::<SleepState>,
        }
    }

    pub fn set_default_settings(&mut self) {
        todo!()
    }

    pub fn go_to_normal_state(mut self) -> Option<Hc12<P, S, D, NormalState>> {
        self.set_pin.set_high().ok()?;
        self.delay.delay_ms(12);
        Some(Hc12 {
            set_pin: self.set_pin,
            serial: self.serial,
            delay: self.delay,
            state: PhantomData::<NormalState>,
        })
    }
}

impl<P, S, D> Hc12<P, S, D, CommandState>
where
    P: OutputPin,
    D: DelayMs<u16>,
    S: Read<u8> + Write<u8>,
{
    pub fn wake_up(mut self) -> Option<Hc12<P, S, D, CommandState>> {
        self.set_pin.set_low().ok()?;
        self.delay.delay_ms(50);
        Some(Hc12 {
            set_pin: self.set_pin,
            serial: self.serial,
            delay: self.delay,
            state: PhantomData::<CommandState>,
        })
    }
}

impl<P, S, D> Write<u8> for Hc12<P, S, D, NormalState>
where
    P: OutputPin,
    D: DelayMs<u16>,
    S: Write<u8>,
{
    type Error = Error;

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        match self.serial.flush() {
            Ok(()) => return Ok(()),
            Err(_) => return Err(nb::Error::Other(Error::Write)),
        }
    }

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        match self.serial.write(word) {
            Ok(()) => return Ok(()),
            Err(_) => return Err(nb::Error::Other(Error::Write)),
        }
    }
}

impl<P, S, D> Read<u8> for Hc12<P, S, D, NormalState>
where
    P: OutputPin,
    D: DelayMs<u16>,
    S: Read<u8>,
{
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        match self.serial.read() {
            Ok(c) => return Ok(c),
            Err(_) => Err(nb::Error::Other(Error::Read)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::{
        pin::{self, State},
        serial,
    };

    #[test]
    fn bare_at_some() {
        let expectations = [
            serial::Transaction::write('A' as u8),
            serial::Transaction::write('T' as u8),
            serial::Transaction::write('\r' as u8),
            serial::Transaction::write('\n' as u8),
            serial::Transaction::read('O' as u8),
            serial::Transaction::read('k' as u8),
            serial::Transaction::read('\r' as u8),
            serial::Transaction::read('\n' as u8),
        ];
        let pin_expectations = [
            pin::Transaction::set(State::Low),
            pin::Transaction::set(State::High),
        ];
        let serial = serial::Mock::<u8>::new(&expectations);
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
            serial::Transaction::write('A' as u8),
            serial::Transaction::write('T' as u8),
            serial::Transaction::write('\r' as u8),
            serial::Transaction::write('\n' as u8),
            serial::Transaction::read('B' as u8),
            serial::Transaction::read('l' as u8),
            serial::Transaction::read('a' as u8),
            serial::Transaction::read('\r' as u8),
            serial::Transaction::read('\n' as u8),
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

    #[test]
    fn test_write_normal() {
        let expectations = [
            serial::Transaction::write('S' as u8),
            serial::Transaction::write('o' as u8),
            serial::Transaction::write('m' as u8),
            serial::Transaction::write('e' as u8),
            serial::Transaction::write('D' as u8),
            serial::Transaction::write('a' as u8),
            serial::Transaction::write('t' as u8),
            serial::Transaction::write('a' as u8),
        ];
        let serial = serial::Mock::new(&expectations);
        let delay = embedded_hal_mock::delay::MockNoop;
        let set_pin = embedded_hal_mock::pin::Mock::new(&[]);
        let mut hc12 = Hc12 {
            set_pin,
            delay,
            serial,
            state: PhantomData::<NormalState>,
        };
        for ch in b"SomeData".iter() {
            match hc12.write(*ch) {
                Ok(_) => {}
                Err(_) => {
                    unreachable!();
                }
            }
        }
        let (mut p, _, mut s) = hc12.release();
        p.done();
        s.done();
    }

    #[test]
    fn test_channel_get_freq_default() {
        let chan = Channel::default();
        assert_eq!(433.4f32, chan.get_freq_mhz());
    }

    #[test]
    fn test_channel_get_freq_100() {
        let chan = Channel { channel: 100 };
        assert_eq!(473.0f32, chan.get_freq_mhz());
    }

    #[test]
    fn test_channel_get_freq_21() {
        let chan = Channel { channel: 21 };
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
}
