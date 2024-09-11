use std::io;
use std::io::Write;
use reqwest;

fn get_user_input() -> String {
    print!("Please enter a valid ISBN-13: ");
    // this line is only needed to flush after using print!, with println! this is not needed
    std::io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim()
}

fn main() {

    let prefix = get_user_input();

    let url = format!( "http://doi.crossref.org/getPrefixPublisher/?prefix={}", &prefix);

    let resp = match reqwest::blocking::get(url) {
         Ok(resp) => resp.text().unwrap(),
         Err(err) => panic!("Error: {}", err)
     };


    let doc = roxmltree::Document::parse(&resp);

    let elem = doc.expect("Failed to retrieve document");

    println!("{:?}", elem.descendants()
             .find(|n| n.has_tag_name("publisher_name"))
             .expect("Failed to retrieve tag data")
             .text()
             .unwrap_or_default())
}
