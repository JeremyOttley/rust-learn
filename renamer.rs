fn is_image(s: &str) -> bool {
        match s {
            s if s.ends_with(".jpg") || 
            s.ends_with(".png") || 
            s.ends_with(".bmp") || 
            s.ends_with(".jpeg") => true,
            _ => false,
        }
}

fn get_images() {
    x
}


fn rename() {
    x
}
//////////////////////////
use rand::Rng;
use rand::distributions::{Alphanumeric};

fn generate_random_string(length: usize) -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(length)
        .map(|byte| byte as char)
        .collect::<String>()
}

fn frobnicate(files: [&'static str]) -> [&'static str] {
    let mut my_vec: Vec<&str> = Vec::new();
    for file in files {
        let new_name = format!("{}.{}", generate_random_string(10), file.split(".").last().unwrap());
        my_vec.push(&new_name)
    }
}


// Example usage
fn main() {
    let files = &["file.png", "file.bmp", "file.webm"];
    let d = frobnicate(files);
    println!("Generated random string: {}", d);
}
