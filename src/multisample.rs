/// MSAA level
#[derive(PartialEq)]
pub enum Multisample {
    /// No anti-aliasing
    X1,
    /// 4X MSAA
    X4,
}

impl Default for Multisample {
    fn default() -> Self {
        Self::X1
    }
}
