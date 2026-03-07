#[derive(PartialEq)]
pub enum Multisample {
    X1,
    X4,
}

impl Default for Multisample {
    fn default() -> Self {
        Self::X1
    }
}
