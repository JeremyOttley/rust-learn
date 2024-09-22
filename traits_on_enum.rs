#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SquareContent {
    Empty,
    X,
    O,
}

// assigns a Default value for the enum
impl Default for SquareContent {
    fn default() -> Self {
        SquareContent::Empty
    }
}

// `From` needs type information
// convert from a u8 to a SquareContent
impl From<u8> for SquareContent {
    fn from(value: u8) -> Self {
        match value {
            0 => SquareContent::Empty,
            1 => SquareContent::X,
            2 => SquareContent::O,
            v: u8 => panic!("cannot convert {} to square content", v),
        }
    }
}
