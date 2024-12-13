mod monitor;
mod status;

use std::env;
use std::fs;

fn main() {
    //Check if the filenmae is provided as an argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];

    println!("Reading URLs from file: {}", filename);

    let contents = fs::read_to_string(filename).expect("Could not read file");

    println!("File contents:\n{}", contents);

    let urls: Vec<String> = contents.lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                println!("Processing URL: {}", trimmed);
                Some(trimmed.to_string())
            } else {
                None
            }
        })
        .collect();

    println!("Final URLs to monitor: {:?}", urls);

    let monitor = monitor::Monitor::new(urls);
    monitor.run();

}
