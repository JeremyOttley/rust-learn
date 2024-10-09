use std::path::{Path};

fn parse_filename(filename: &str) -> (String, Option<String>) {
    let path = Path::new(filename);
    let stem = path.file_stem();
    let extension = path.extension();

    (
        stem.map_or_else(String::new, |s| s.to_string_lossy().into_owned()),
        extension.map(|ext| ext.to_string_lossy().to_string()),
    )
}

fn main() {
    let filename = "example.txt";
    let (basename, extension) = parse_filename(filename);

    println!("Basename: {}", basename);
    println!("Extension: {}", extension.unwrap_or_default());
}
