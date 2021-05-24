//! Hc12 driver
//! This driver implements normal, config and sleep functionality of the hc12 module.

#![cfg_attr(not(test), no_std)]
#![deny(unsafe_code)]
#![deny(missing_docs)]

/// Hc12 settings
pub mod settings;

/// Hc12 driver
pub mod hc12;

/// Crate error
#[derive(Debug)]
pub enum Error {
    /// Read error
    Read,
    /// Write error
    Write,
    /// Invalid baud rate
    InvalidBaudRate,
}
