use chrono::{DateTime, Utc};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("Backup create at: {}", backup_file("dwm"));
    
    Ok(())
}

fn backup_file(filename: &str) -> String {

    let now: DateTime<Utc> = Utc::now();

    let formatted_date = now.format("%d-%m-%Y").to_string();

    format!("{}-{}", formatted_date, filename)
}
