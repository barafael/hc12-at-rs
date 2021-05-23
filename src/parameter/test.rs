use core::convert::TryFrom;

use crate::parameter::baudrate::BaudRate;

#[test]
fn parse_baudrate_from_i32() {
    let baudrates = [1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200];
    let expected = [
        BaudRate::Bps1200,
        BaudRate::Bps2400,
        BaudRate::Bps4800,
        BaudRate::Bps9600,
        BaudRate::Bps19200,
        BaudRate::Bps38400,
        BaudRate::Bps57600,
        BaudRate::Bps115200,
    ];
    let result: Vec<BaudRate> = baudrates
        .iter()
        .map(|x| BaudRate::try_from(*x).unwrap())
        .collect();
    assert_eq!(&expected, &result[..]);
}
