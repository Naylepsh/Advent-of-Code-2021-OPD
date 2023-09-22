use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "src/input";

    // Open the file in read-only mode.
    let file = File::open(file_path)?;

    // Create a buffered reader for efficient reading.
    let reader = BufReader::new(file);

    // Initialize variables.
    let mut current_window: Vec<i32> = Vec::new();
    let mut previous_window: Vec<i32> = Vec::new();
    let mut counts = 0;

    for line in reader.lines() {
        let line = line?;

        if let Ok(current_value) = line.parse::<i32>() {
            // Add the current value to the current window.
            current_window.push(current_value);

            // Ensure the current window size is limited to 3.
            if current_window.len() > 3 {
                current_window.remove(0); // Remove the oldest value.
            }

            // When the current window contains 3 measurements and the previous window also has 3, compare the sum.
            if current_window.len() == 3 && previous_window.len() == 3 {
                let current_sum: i32 = current_window.iter().sum();
                let previous_sum: i32 = previous_window.iter().sum();

                if current_sum > previous_sum {
                    counts += 1;
                }
            }
            previous_window = current_window.clone();
        }
    }

    // Print the final count.
    println!("Count: {}", counts);

    Ok(())
}