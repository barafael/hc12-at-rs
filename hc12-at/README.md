# hc12

Driver for the hc-12 radio transceiver serial module.

This module can be configured using AT commands while the SET pin is pulled low. This driver takes an OutputPin, Serial Port and DelayMs from embedded-hal and offers a somewhat convenient interface to interact with the hc12 module.

The driver uses type-state programming to discriminate between the operation states of the hc12: Idle, Sleeping, Configuration.

Datasheet: https://www.elecrow.com/download/HC-12.pdf

# Example

```rust
let hc12 = Hc12::new(serial, set_pin, delay);
let mut hc12 = hc12.into_configuration_mode().debugless_unwrap();

let mut buffer = [0u8; 16];

let ok = hc12.is_ok();
assert!(ok);

let version = hc12.get_version(&mut buffer);
assert_eq!(version, b"VERSION-42\r\n");

let params = hc12.get_parameters().unwrap();
assert_eq!(
    Parameters {
        baud_rate: BaudRate::Bps115200,
        channel: Channel(42),
        power: TransmissionPower(1),
        mode: Mode::Fu2,
    },
    params
);

let mut hc12 = hc12.into_normal_mode().debugless_unwrap();

hc12.write_buffer(b"some data AT AT\r\n").unwrap();

let (mut serial, mut set_pin, _) = hc12.release();
```

(^: see hc12::test::usage_from_readme)
