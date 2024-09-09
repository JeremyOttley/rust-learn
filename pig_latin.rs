fn main() {
    println!("{}", pig_latin::convert("Jeremy"));
}

mod pig_latin {
    fn is_vowel(c: &char) -> bool {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 
            'A' | 'E' | 'I' | 'O' | 'U' => true,
            _ => false
        }
    }
    
    pub fn convert(word: &str) -> String {
        let max = word.len() - 1;
        if is_vowel(&word.chars().nth(0).unwrap()) {
            return  format!("{}{}", &word, "yay");
        } else {
            return format!("{}{}{}", &word[1..=max], &word.chars().nth(0).unwrap(), "ay");
        }        
    }

}
