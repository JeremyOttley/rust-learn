fn main() {

    for i in 1..=20 {
        fizz_buzz(i);   
    }

}

fn mult_of_3(n: i32) -> bool {
    return n % 3 == 0
}

fn mult_of_5(n: i32) -> bool {
    return n % 5 == 0
}

fn fizz_buzz(n: i32) {

    if mult_of_3(n) && mult_of_5(n) {
        println!("{}", "FizzBuzz");
    } else if mult_of_3(n) {
        println!("{}", "Fizz");
    } else {
        println!("{}", "Buzz");
    }

} 
