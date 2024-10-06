fn main() {
    let f = "file.jpeg";
    println!("{}", file_ext(f).unwrap_or("No file extension found"));
}

fn file_ext(name: &str) -> Option<&str> {
    name.split(".").last()
}
