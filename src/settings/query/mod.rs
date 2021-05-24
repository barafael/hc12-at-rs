pub(crate) mod baudrate;
pub(crate) mod channel;
pub(crate) mod mode;
pub(crate) mod transmission_power;

#[cfg(test)]
mod test;

/// Make an AT query
pub trait MakeQuery {
    /// Return the AT query of this type
    fn make_query(buffer: &mut [u8; 7]);
}
