use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

const SPACE_WIDTH: usize = 3;

// Function to calculate checksum of a string (line)
fn calculate_checksum(line: &str) -> u8 {
    line.bytes().fold(0, |acc, byte| acc.wrapping_add(byte))
}

fn main() -> io::Result<()> {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if the correct number of arguments are provided
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return Ok(());
    }

    // Open the input file for reading
    let input_filename = &args[1];
    let input_path = Path::new(input_filename);
    let input_file = File::open(&input_path)?;

    // Prepare the output file name by changing the extension
    let output_filename = match input_path.extension() {
        Some(_) => input_path.with_extension("out"),
        None => input_path.with_extension("out"),
    };
    let mut output_file = File::create(output_filename)?;

    // Create a buffered reader for the input file
    let reader = io::BufReader::new(input_file);

    // Process each line of the input file
    for line in reader.lines() {
        let line = line?;
        
        // Calculate the checksum for the line
        let checksum = calculate_checksum(&line);

        // Print the checksum followed by the original line in the output file
        writeln!(output_file, "{:02X}{:space$}{}", checksum, "", line, space = SPACE_WIDTH)?;
    }

    println!("Output written successfully.");
    Ok(())
}
