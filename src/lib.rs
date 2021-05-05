#![cfg_attr(not(test), no_std)]

mod config;

pub mod hc12;

#[derive(Debug)]
pub enum Error {
    Read,
    Write,
    InvalidChannel(u8),
    InvalidBaudRate,
}
