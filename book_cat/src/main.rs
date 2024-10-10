use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::error::Error;

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?; 

    for book in books {
        writeln!(file, "{},{},{}", book.title, book.author, book.year)?; // Write each book to the file
    }
    
    Ok(()) // Indicate successful completion
}

// Function to load books from a file
fn load_books(filename: &str) -> Result<Vec<Book>, Box<dyn Error>> {
    let file = File::open(filename)?; // Open the specified file
    let reader = BufReader::new(file);
    let mut books = Vec::new();

    for line in reader.lines() {
        let line = line?; // Get each line from the file
        let parts: Vec<&str> = line.split(',').collect(); // Split the line by commas
        if parts.len() == 3 {
            let title = parts[0].to_string();
            let author = parts[1].to_string();
            let year = parts[2].parse::<u16>()?; // Parse the year as u16
            books.push(Book { title, author, year }); // Create a new Book and add it to the vector
        }
    }
    Ok(books) // Return the vector of books
}

fn main() -> Result<(), Box<dyn Error>> {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
        Book { title: "The Great Gatsby".to_string(), author: "F. Scott Fitzgerald".to_string(), year: 1925 },
    ];

    
    save_books(&books, "books.txt")?;
    println!("Books saved to file.");

   
    let loaded_books = load_books("books.txt")?;
    println!("Loaded books:");
    
    
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }

    Ok(())
}
