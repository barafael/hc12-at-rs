//! Hc12 driver
//! This driver implements normal, config and sleep functionality of the hc12 module.

#![cfg_attr(not(test), no_std)]
#![deny(unsafe_code)]
#![deny(missing_docs)]

/// Configuration data structures
mod parameter;

/// Hc12 driver
pub mod hc12;

/// Command datastructures
pub mod command;

/// Query datastructures
pub mod query;

/// Hc12 response parser
pub mod response_parser;

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
