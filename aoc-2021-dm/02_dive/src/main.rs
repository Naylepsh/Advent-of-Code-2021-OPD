use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "src/input";

    // Open the file in read-only mode.
    let file = File::open(file_path)?;

    // Create a buffered reader for efficient reading.
    let reader = BufReader::new(file);

    let (mut horizontal, mut depth) = (0_i32, 0_i32);

    for line in reader.lines() {
        let line = line?;
        _reader_command(line, &mut horizontal, &mut depth);
    }

    println!("{}", horizontal * depth);

    Ok(())
}

fn _reader_command(line: String, horizontal: &mut i32, depth: &mut i32) {
    let accepted_commands = vec!["forward", "down", "up"];

    let parts: Vec<&str> = line.split_whitespace().collect();

    // Now, you can destructure the vector into a command and value.
    if let [command, value] = parts.as_slice() {
        if let Ok(int_value) = value.parse::<i32>() {
            if accepted_commands.contains(&command) {
                if command == &"forward" {
                    *horizontal += int_value;
                } else if command == &"down" {
                    *depth += int_value;
                } else if command == &"up" {
                    *depth -= int_value;
                }
            }
        }
    } else {
        println!("Invalid input format");
    }
}
