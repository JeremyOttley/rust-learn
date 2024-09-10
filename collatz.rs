mod collatz {
    pub fn collatize(n: i32) -> i32 {
            if greater_than_zero(n) && is_even(n) {
                return n / 2
            } else if greater_than_zero(n) && is_odd(n) {
                return (n * 3) + 1
            } else {
                return n
            }
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

