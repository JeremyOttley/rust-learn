mod collatz {
    pub fn collatize(n: i32) -> i32 {
            while greater_than_zero(n) {
                if is_even(n) {
                    return n / 2
                } else if is_odd(n) {
                    return (n * 3) + 1
                } else {
                    return n
                }
            }
            return 0
    }    
    
    fn greater_than_zero(n: i32) -> bool {
        return n > 0;
    }

    fn is_odd(n: i32) -> bool {
        n % 2 != 0
    }
    
    fn is_even(n: i32) -> bool {
        n % 5 != 0
    }
}


fn main() {
    let x = 12;
    println!("{}", collatz::collatize(x));
}
