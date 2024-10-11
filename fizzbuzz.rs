fn main() {

    for i in 1..=20 {
        fizz_buzz(i);   
    }

}

fn mult_of_5(n: u32) -> bool {
    return n % 5 == 0
}

fn mult_of_15(n: u32) -> bool {
    mult_of_3(n) && mult_of_5(n)
}

fn fizz_buzz(n: u32) -> () {
    match n {
        n if mult_of_15(n) => println!("FizzBuzz!"),
        n if mult_of_5(n) => println!("Buzz!"), 
        n if mult_of_3(n) => println!("Fizz!"), 
        _ => println!("{}", n),
    }
}
