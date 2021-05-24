#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Fu1,
    Fu2,
    Fu3,
    Fu4,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Fu3
    }
}
