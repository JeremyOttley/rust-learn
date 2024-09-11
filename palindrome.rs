fn main() {
	println!("{}", palindrome::try_parse("racecar"));
}

mod palindrome {
	pub fn try_parse(s: &str) -> bool {
	    s == s.chars() // string to chars
	    .rev() // reverse collection
	    .collect::<String>() // collect as String
	}
}
