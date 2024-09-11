fn main() {
    println!("{}", factorial(4));
}

fn factorial(n: u32) -> u32 {
    match n {
        n if n > 0 => n * factorial(n - 1),
        _ => 1,
    }
}
