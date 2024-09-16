fn main() {
    println!("{:?}", count_vowels::count("Jeremy"));
}

mod count_vowels {
    fn is_vowel(c: &char) -> bool {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 
            'A' | 'E' | 'I' | 'O' | 'U' => true,
            _ => false
        }
    }

    // Cleaner code with `matches!` macro
    // fn is_vowel(c: &char) -> bool {
    //     matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 
    //         'A' | 'E' | 'I' | 'O' | 'U')
    // }
    
    pub fn count(word: &str) -> u32 {
        let vowels: Vec<char> = word.chars().filter(|c| is_vowel(c)).collect();
        return vowels.len() as u32;
    }

}
