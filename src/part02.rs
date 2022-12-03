use std::io;

///////////////////////
// Guessing Game
///////////////////////

pub fn run() {
    // guess_game_v1();

    // Allow multiple guesses with looping
    // guess_game_v2();

    // Handling Invalid input
    guess_game_v3();
}

fn guess_game_v1() {
    // decalare variable to store user input
    // this is a mutable variable
    let mut guess = String::new();

    // take user input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // We passed mutable variable to `read_line` method so that the method can
    // change the variable's content.

    // The & indicates that this argument is a reference, which lets multiple
    // parts of your code to access one piece of data  without needing to copy
    // that data into memory multiple times.

    // The `read_line` returns a `Result` type value. Its an enumeration which
    // has two variants `Ok` & `Err`.

    // If the instance of `Result` type is an `Err` value, `expect` will cause
    // the program to crash.
    println!("Your guess is {guess}");


}

fn guess_game_v2() {
    // We want to generate random numbers for our guessing game.
    // Rust by default don't have any random number generator library, so
    // We will add one to our project. We can do it using Cargo.toml file.
    // Add following line to you `Cargo.toml` file dependency section.
    // `rand = "0.8.5"`

    use rand::{thread_rng, Rng};
    let secret_number = rand::thread_rng().gen_range(0..=100);

    println!("The secret number is {secret_number}");

    // loop to take multiple user guesses
    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guess: {guess}");

        // define a unsigned integer to take integer input
        // this will overshadow the earlier defined guess variable
        let guess: u32 = guess.trim().parse().expect("Plese enter a number!");

        // We need to trim the user input as it contains unneccessary `\n`.
        // This will cause parsing of input as integer to fail.

        // The parse method on strings converts a string to another type.
        // We need to tell Rust the exact number type we want by using `let guess: u32`
        // Because it might fail, the parse method returns a Result type, much as
        // the read_line method does.

        // If parse returns an Err Result variant because it couldn’t create a
        // number from the string, the `expect` call will crash the game and print
        // the message we give it.

        use std::cmp::Ordering;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // quit if your guess is right
            }
        }
    }
}

fn guess_game_v3() {
    // We want to generate random numbers for our guessing game.
    // Rust by default don't have any random number generator library, so
    // We will add one to our project. We can do it using Cargo.toml file.
    // Add following line to you `Cargo.toml` file dependency section.
    // `rand = "0.8.5"`

    use rand::{thread_rng, Rng};
    let secret_number = rand::thread_rng().gen_range(0..=100);

    println!("The secret number is {secret_number}");

    // using loop to take multiple user guesses
    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guess: {guess}");

        // Rathen than crashing the program when the user inputs a non-number,
        // make the game ignore a non-number so that user can continue guessing.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // We need to trim the user input as it contains unneccessary `\n`.
        // This will cause parsing of input as integer to fail.

        // The parse method on strings converts a string to another type.
        // We need to tell Rust the exact number type we want by using `let guess: u32`
        // Because it might fail, the parse method returns a Result type, much as
        // the read_line method does.

        // If parse returns an Err Result variant because it couldn’t create a
        // number from the string, the `expect` call will crash the game and print
        // the message we give it.

        use std::cmp::Ordering;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // quit if your guess is right
            }
        }
    }
}
