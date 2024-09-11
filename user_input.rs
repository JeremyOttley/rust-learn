use std::io;
use std::io::Write;

fn get_user_input() -> Option<String> {

    print!("Please enter a valid ISBN-13: ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if input.is_empty() {
        return None;
    }
    	return Some(input);
}

fn main() {
	//use expect to unwrap an option like withDefault in Haskell or Elm
    println!("{:?}", get_user_input().expect("Failed to get input").trim());
}
