/// Baud rate datastructures
pub mod baudrate;
/// Communication channel datastructures
pub mod channel;
/// Operational mode datastructures
pub mod mode;
/// Transmission power
pub mod transmission_power;

/// HC-12 parameters
pub mod parameters;

pub(crate) const OK_QUERY: [u8; 4] = *b"AT\r\n";
pub(crate) const OK_RESPONSE: [u8; 4] = *b"OK\r\n";

pub(crate) const QUERY_PARAMS_COMMAND: [u8; 7] = *b"AT+RX\r\n";

pub(crate) const SLEEP_COMMAND: [u8; 10] = *b"AT+SLEEP\r\n";
pub(crate) const SLEEP_RESPONSE: [u8; 10] = *b"OK+SLEEP\r\n";

pub(crate) const RESET_SETTINGS_COMMAND: [u8; 12] = *b"AT+DEFAULT\r\n";
pub(crate) const RESET_SETTINGS_RESPONSE: [u8; 12] = *b"OK+DEFAULT\r\n";

pub(crate) const VERSION_QUERY: [u8; 6] = *b"AT+V\r\n";
