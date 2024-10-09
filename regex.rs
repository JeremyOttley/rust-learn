use regex::Regex;

fn main() {
    let re = Regex::new(r"\[(?<newfilename>\w+)\]").unwrap();
    let Some(new_file_name) = re.captures("20D4EB1 [GA4D84F4].mp4") else {
        println!("no match!");
        return;
    };
    let x = format!("{}.mp4", &new_file_name["newfilename"]);
    println!("File renamed to {}", x);
}
