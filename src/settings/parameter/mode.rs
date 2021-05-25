/// Operational mode
#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    /// Function 1
    Fu1,
    /// Function 2
    Fu2,
    /// Function 3
    Fu3,
    /// Function 4
    Fu4,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Fu3
    }
}
