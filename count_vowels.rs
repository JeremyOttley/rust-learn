fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 
        'A' | 'E' | 'I' | 'O' | 'U' => true,
        _ => false
    }
}
