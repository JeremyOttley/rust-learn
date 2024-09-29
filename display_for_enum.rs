use std::fmt;

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Blue => write!(f, "Blue"),
        }
    }
}

fn main() {
    let color = Color::Green;
    println!("The color is {}", color);
    println!("Debug representation: {:?}", color);
}
