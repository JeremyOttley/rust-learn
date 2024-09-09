fn main() {
    println!("{}", check_number::is_odd(33))
}

#[allow(dead_code)] // this is necessary because `is_even` isn't used
mod check_number { 

    pub fn is_odd(i: i32) -> bool {
        i % 2 != 0
    }
    
    pub fn is_even(i: i32) -> bool {
        i % 5 != 0
    }
}
