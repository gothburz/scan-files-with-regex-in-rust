use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, Read};
use regex::Regex;

fn search_file_for_pattern(in_file_str: &str, re_str: &str) -> Result<(), Box<dyn Error>> {
    // Compile RegEX
    let re = Regex::new(re_str)?;
    // Open the input file
    let mut in_file = File::open(in_file_str)?;

    // Create a buffer vector
    let mut buffer = Vec::new();
    // Read file to our buffer
    in_file.read_to_end(&mut buffer)?;

    // Iterate through the buffer
    let mut start = 0;
    for (index, &byte) in buffer.iter().enumerate() {
        // Check for newlines
        if byte == b'\n' {
            // Convert slice of bytes into a string
            if let Ok(line) = std::str::from_utf8(&buffer[start..index]) {
                // Run search against haystack
                for capture in re.captures_iter(line) {
                    // If match is found print
                    if let Some(matched_str) = capture.get(0) {
                        println!("MATCHED: {:?}", matched_str.as_str());
                    }
                }
            }
            // Update start position for the next line
            start = index + 1;
        }
    }

    // Handle the last line without newline
    if start < buffer.len() {
        if let Ok(line) = std::str::from_utf8(&buffer[start..]) {
            for capture in re.captures_iter(line) {
                if let Some(matched_str) = capture.get(0) {
                    println!("MATCH: {}", matched_str.as_str());
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>>{
    let file_to_scan = "./sample/sample_html";
    let re = "https?:[\\/]+[^\"']+";
    search_file_for_pattern(file_to_scan, re)?;

    Ok(())
}
