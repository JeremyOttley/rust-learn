fn factorial(n: u32) -> u32 {
    assert!(n >= 1);
    if n == 1 {
        n
    } else {
        n * factorial(n - 1)
    }
}
