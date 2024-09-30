use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

struct Car {
    make: String,
    model: String,
    year: String,
}

fn main() -> io::Result<()> {
    let mut car = Car {
        make: String::new(),
        model: String::new(),
        year: String::new(),
    };

    // Collect user input
    println!("Enter your car's make:");
    io::stdin().read_line(&mut car.make)?;
    car.make = car.make.trim().to_string(); // Remove newline

    println!("Enter your car's model:");
    io::stdin().read_line(&mut car.model)?;
    car.model = car.model.trim().to_string(); // Remove newline

    println!("Enter your car's year:");
    io::stdin().read_line(&mut car.year)?;
    car.year = car.year.trim().to_string(); // Remove newline

    // Save the car information to a file
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("user_info.txt")?;
    writeln!(file, "Make: {}\nModel: {}\nYear: {}", car.make, car.model, car.year)?;

    // Read and print the content from the file
    let mut file = File::open("user_info.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Print the content on the screen
    println!("Content of user_info.txt:");
    println!("{}", contents);

    Ok(())
}
