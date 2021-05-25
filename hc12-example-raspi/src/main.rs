use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::uart::{Parity, Uart};

fn main() -> Result<(), ()> {
    let uart = Uart::new(9600, Parity::None, 8, 1).unwrap();

    let set_pin = Gpio::new().unwrap().get(18).unwrap().into_output();

    let hc12 = hc12_at::hc12::Hc12::new(uart, set_pin, linux_embedded_hal::Delay);

    let mut hc12 = match hc12.into_configuration_mode() {
        Ok(r) => r,
        Err(_) => panic!(),
    };

    assert!(hc12.is_ok());

    let mut buffer = [0u8; 64];
    let result = hc12.get_version(&mut buffer);
    println!("{:?}", std::str::from_utf8(&result).unwrap());

    let params = hc12.get_parameters().unwrap();

    println!("{:#?}", params);

    println!("{:?}", params.get_air_baud_rate());

    println!(
        "{:?}",
        params.get_air_baud_rate().get_wireless_sensitivity_dbm()
    );

    let mut hc12 = match hc12.into_normal_mode() {
        Ok(r) => r,
        Err(_) => panic!(),
    };

    loop {
        hc12.write_buffer(b"hello hc12\r\n").unwrap();
        thread::sleep(Duration::from_millis(500));
        //let a = block!(hc12.read()).unwrap();
        //let _ = block!(hc12.write(a));
    }
}
