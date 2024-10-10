fn main() {
    debug_assert!(acronym("Jeremy Mark Ottley") == "JMO");
}


fn acronym(phrase: &str) -> String {
    phrase
    .chars()
    .filter(|c| { c.is_uppercase() }).collect::<String>()
}

