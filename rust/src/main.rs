mod day1;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Call the day1 function
    if let Err(e) = day1::challenge() {
        eprintln!("Error in day1 function: {}", e);
        return Err(e);
    }

    Ok(())
}
