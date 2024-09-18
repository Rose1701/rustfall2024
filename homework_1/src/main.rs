//Assignment 1: Temperature Converter

const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + FREEZING_POINT_F
}

fn main() {
    let mut temperature_f = 32.0; // Starting temperature in Fahrenheit.

    // Convert and print the initial temperature.
    let temperature_c = fahrenheit_to_celsius(temperature_f);
    println!("{}°F is {:.2}°C", temperature_f, temperature_c);

    // Loop to convert and print the next 5 integer temperatures.
    for i in 1..6 {
        temperature_f += 1.0; // Incrementing the temperature.
        let temperature_c = fahrenheit_to_celsius(temperature_f);
        println!("{}°F is {:.2}°C", temperature_f, temperature_c);
    }
}

/*Assignment 2: Number Analyzer

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // Create an array of 10 integer numbers.
    let numbers: [i32; 10] = [10, 17, 23, 42, 3, 26, 30, 7, 8, 12];

    // Going through the array and analyze each number.
    for &number in &numbers {
        if number % 3 == 0 && number % 5 == 0 {
            println!("{}: FizzBuzz", number);
        } else if number % 3 == 0 {
            println!("{}: Fizz", number);
        } else if number % 5 == 0 {
            println!("{}: Buzz", number);
        } else {
            if is_even(number) {
                println!("{}: Even", number);
            } else {
                println!("{}: Odd", number);
            }
        }
    }

    // Calculate the sum of all numbers using a while loop.
    let mut sum = 0;
    let mut index = 0;

    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("Sum of all numbers: {}", sum);

    // Finding the largest number in the array.
    let mut largest = numbers[0];

    for &number in &numbers {
        if number > largest {
            largest = number;
        }
    }
    println!("Largest number: {}", largest);
}*/

/*Assignment 3: Guessing Game

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0 // Correct guess.
    } else if guess > secret {
        1 // If guess is too high.
    } else {
        -1 // If guess is too low.
    }
}

fn main() {
    let secret = 42; // Secret number.
    let mut guess: i32;
    let mut attempts = 0;
    
    loop {
        // Simulate user input by setting a guess variable
        // You can change this number to test different scenarios
        guess = 40 + attempts; // Just an example to vary the guess
        
        attempts += 1; // Increment attempt count
        
        // Check the guess
        match check_guess(guess, secret) {
            0 => {
                println!("Congratulations! You've guessed the correct number: {}", guess);
                break; // Exit the loop if the guess is correct
            },
            1 => println!("Your guess of {} is too high.", guess),
            -1 => println!("Your guess of {} is too low.", guess),
            _ => unreachable!(), // Should never reach here
        }
    }
    
    println!("It took you {} attempts to guess the secret number.", attempts);
} */




