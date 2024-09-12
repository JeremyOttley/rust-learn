mod acronym {
    use std::iter::FromIterator;

    fn is_uppercase(c: char) -> bool {
        c.is_ascii_uppercase()
    }

    fn string_of_chars(chars: Vec<char>) -> String {
        String::from_iter(chars)
    }

    pub fn abbreviate(s: &str) -> String {
        let chars: Vec<char> = s.chars()
            .filter(|&c| is_uppercase(c))
            .collect();
        string_of_chars(chars)
    }
}

