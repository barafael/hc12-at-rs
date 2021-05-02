#![no_std]

mod config;

pub mod hc12;

pub enum Error {
    Read,
    Write,
    InvalidChannel(u8),
    InvalidBaudRate,
}
