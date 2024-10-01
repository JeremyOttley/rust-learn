use regex::Regex;

fn main() {
    let re = Regex::new(r"(\d{3}-?\d{1}-?\d{4}-?\d{4}-?(?<check>\d{1}|\w{1}))").unwrap();
    let Some(check_digit) = re.captures("978-1-4780-9418-X") else {
        println!("no match!");
        return;
    };
    println!("Check digit is: {}", &check_digit["check"]);
}
