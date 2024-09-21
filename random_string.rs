use rand::Rng;
use rand::distributions::{Alphanumeric};

fn generate_random_string(length: usize) -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(length)
        .map(|byte| byte as char)
        .collect::<String>()
}

// Example usage
fn main() {
    let random_string = generate_random_string(10);
    println!("Generated random string: {}.jpg", random_string);
}
