use std::io;
use std::io::Write;

fn get_user_input() -> Option<String> {

    print!("Please enter a valid ISBN-13: ");
    // this line is only needed to flush after using print!, with println! this is not needed
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

//978-1-4780-2453-8
