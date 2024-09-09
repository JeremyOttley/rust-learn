fn palindrome(s: &str) -> bool {
    s == s.chars() // string to chars
    .rev() // reverse collection
    .collect::<String>() // collect as String
}
