mod monitor;
mod status;

use std::env;
use std::fs;

fn main() {
    // Check if the filename is provided as an argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];

    // Debug: Print the filename being used
    println!("Reading URLs from file: {}", filename);

    // Read the URLs from the specified file
    let contents = fs::read_to_string(filename).expect("Could not read file");
    
    // Debug: Print the contents of the file
    println!("File contents:\n{}", contents);

    let urls: Vec<String> = contents.lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                println!("Processing URL: {}", trimmed); // Debugging each URL
                Some(trimmed.to_string())
            } else {
                None
            }
        })
        .collect();

    // Debug: Print the URLs read from the file
    println!("Final URLs to monitor: {:?}", urls);

    // Create a Monitor instance and run it
    let monitor = monitor::Monitor::new(urls);
    monitor.run();
}