#![no_std]
#![no_main]

use embedded_hal::digital::v2::OutputPin;
use hc12_at::hc12::Hc12;
use panic_halt as _;

use gd32vf103xx_hal::serial::Serial;
use gd32vf103xx_hal::{pac, prelude::*, serial::Config, time::Bps};

use debugless_unwrap::DebuglessUnwrap;

use riscv_rt::entry;

struct MySerial<TX, RX> {
    tx: TX,
    rx: RX,
}

impl<TX, RX: embedded_hal::serial::Read<u8>> embedded_hal::serial::Read<u8> for MySerial<TX, RX> {
    type Error = RX::Error;

    fn read(&mut self) -> Result<u8, nb::Error<Self::Error>> {
        self.rx.read()
    }
}

impl<TX: embedded_hal::serial::Write<u8>, RX> embedded_hal::serial::Write<u8> for MySerial<TX, RX> {
    type Error = TX::Error;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.tx.write(word)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.tx.flush()
    }
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();
    let mut afio = dp.AFIO.constrain(&mut rcu);

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpiob = dp.GPIOB.split(&mut rcu);

    let mut led = gpioa.pa2.into_open_drain_output();

    let mut delay = gd32vf103xx_hal::delay::McycleDelay::new(&rcu.clocks);

    let set_pin = gpiob.pb1.into_open_drain_output();
    let config = Config::default().baudrate(Bps(9600));
    let serial = Serial::new(
        dp.USART2,
        (gpiob.pb10, gpiob.pb11),
        config,
        &mut afio,
        &mut rcu,
    )
    .split();
    let ms = MySerial {
        tx: serial.0,
        rx: serial.1,
    };

    let mut hc12 = Hc12::new(ms, set_pin, delay);

    loop {
        hc12.write_buffer(b"Hello World").debugless_unwrap();
        hc12.write(b'\r').debugless_unwrap();
        hc12.write(b'\n').debugless_unwrap();
        hc12.flush().debugless_unwrap();
        led.set_low().debugless_unwrap();
        delay.delay_ms(250);
        led.set_high().debugless_unwrap();
        delay.delay_ms(250);
    }
}
