#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SquareContent {
    Empty,
    X,
    O,
}

impl Default for SquareContent {
    fn default() -> Self {
        SquareContent::Empty
    }
}
