use std::error::Error;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn Error>> {

    leg_lift(5);

    Ok(())
}


fn leg_lift(n: u64) -> () {
    let mut last_update = Instant::now();

    loop {
        // Calculate the elapsed time since the last print
        let time_elapsed = last_update.elapsed();

        // Check if 90 seconds have passed since the last print
        if time_elapsed >= Duration::from_secs(n) {
            println!("Switch legs!");
            
            // Update the last print time
            last_update = Instant::now();
        }

        // Sleep for a short duration to avoid excessive CPU usage
        std::thread::sleep(Duration::from_millis(100));
    }
}
