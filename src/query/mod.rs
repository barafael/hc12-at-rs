pub(crate) mod baudrate;
pub(crate) mod channel;
pub(crate) mod mode;
pub(crate) mod transmission_power;

/// Convert a T to an AT query for it
pub trait ToQuery {
    /// Fill the buffer with the query and return the size (0 if failure). TODO return optional slice.
    fn to_query(buffer: &mut [u8; 16]) -> usize;
}
