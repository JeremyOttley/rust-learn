use std::fmt::;

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument).
        write!(f, "Palette: R:{} G:{} B:{}", 
            self.red, self.green, self.blue)
    }
}

fn main() {

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {

        println!("{}", color);
    }

}
