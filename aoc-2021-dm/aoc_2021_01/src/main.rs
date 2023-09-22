use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "src/input";

    // Open the file in read-only mode.
    let file = File::open(file_path)?;

    // Create a buffered reader for efficient reading.
    let reader = BufReader::new(file);

    // Initialize a variable to store the previous line.
    let mut previous_line = String::new();

    let mut counts = 0;

    for line in reader.lines() {
        let line = line?;

        if let Ok(current_value) = line.parse::<i32>() {
            if let Ok(previous_value) = previous_line.parse::<i32>() {
                if current_value > previous_value {
                    counts += 1;
                }
            }
        }

        previous_line = line.clone();
    }

    // Print the final count.
    println!("Count: {}", counts);

    Ok(())
}