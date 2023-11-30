#![no_std]
#![no_main]

use debugless_unwrap::DebuglessUnwrap;
use embedded_hal::serial::Write;
use hc12_at::hc12::Hc12;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);

    let set_pin = pins.d9.into_output();
    let serial = arduino_hal::default_serial!(peripherals, pins, 9600);
    let delay = arduino_hal::Delay::new();

    let mut hc12 = Hc12::new(serial, set_pin, delay);

    let mut led = pins.d13.into_output();

    loop {
        hc12.write_buffer(b"Hello World").debugless_unwrap();
        hc12.write(b'\r').debugless_unwrap();
        hc12.write(b'\n').debugless_unwrap();
        hc12.flush().debugless_unwrap();
        led.toggle();
    }
}
