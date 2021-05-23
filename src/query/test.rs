use crate::{
    parameter::{
        baudrate::BaudRate, channel::Channel, mode::Mode, transmission_power::TransmissionPower,
    },
    query::MakeQuery,
};

#[test]
fn query_single_param() {
    let mut buffer = [0u8; 7];
    BaudRate::make_query(&mut buffer);
    assert_eq!(b"AT+RB\r\n", &buffer);
    Channel::make_query(&mut buffer);
    assert_eq!(b"AT+RC\r\n", &buffer);
    Mode::make_query(&mut buffer);
    assert_eq!(b"AT+RF\r\n", &buffer);
    TransmissionPower::make_query(&mut buffer);
    assert_eq!(b"AT+RP\r\n", &buffer);
}
