use base64::{Engine as _, engine::general_purpose};

fn main() {
    let input = "Jeremy";
    let encoded = general_purpose::STANDARD.encode(input);

    println!("Encoded string: {}", encoded);

}
