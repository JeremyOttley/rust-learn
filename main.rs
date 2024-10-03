use std::error::Error;

// With Box<dyn Error>, you get runtime polymorphism, meaning the actual error type 
// is only known at runtime, not statically
fn main() -> Result<(), Box<dyn Error>> {


    Ok(())
}
