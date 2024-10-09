use regex::Regex;

fn main() {
    let re = Regex::new(r"\[(?<newfilename>\w+)\]").unwrap();
    let Some(new_file_name) = re.captures("20D4EB1 [GA4D84F4].mp4") else {
        println!("no match!");
        return;
    };
    //println!("Check digit is: {}", &check_digit["check"]);
    println!("File renamed to {:?}.mp4", &new_file_name["newfilename"]);
}
