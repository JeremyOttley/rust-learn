use chrono::prelude::*;
use chrono::Utc;


fn main() {
    let epoch_time: i64 = 1729693293;
    #[allow(deprecated)]
    let dt = Utc.timestamp(epoch_time, 0);
    let formatted_date = dt.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("The ISS will pass above your city at {}", formatted_date)
}
