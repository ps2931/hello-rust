// Rust groups errors into two major categories: recoverable and unrecoverable errors.
// For a recoverable error, such as file not found, we may want to report the issue
// to the user and retry. Unrecoverable errors are symptoms of bugs, like trying to
// access a location beyond the array size, so we want to immediately stop the program.

// Rust does not have exceptions. Instead it has type Result<T, E> for recoverable errors,
// and `panic!` for unrecoverable errors.

pub fn run() {
    recoverable_errors();
}

fn unrecoverable_errors() {
    // A panic can happen if your code has a faulty action or you call panic! manually.

    // Manual call: panic!("crash and burn");

    // Via code
    let v = vec![1,2,3];
    v[99];

}

fn recoverable_errors() {
    // Result<T, E> is an enum having two variants: Ok(T), Err(E).

    use std::fs::File;
    let greeting_file_result = File::open("hello.txt");
    // The return type of File::open is a Result<T, E>
    // In the case where File::open succeeds, the value in the variable greeting_file_result
    // will be an instance of Ok that contains a file handle.

    // Check if call to File::open() was success or failure
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    // Matching on Different Errors
    // Instead of panic!, we want to take different actions for different failure
    // reasons: if File::open failed because the file doesn’t exist, we want to
    // create the file and return the handle to the new file. If File::open failed
    // for any other reason, for example, because we didn’t have permission to
    // open the file—we still want the code to panic!
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // File::create itself can fail, therefore we need a second match expression
    // to check ofr success or failure result.

    // The match expression is very useful but also very much a primitive.
    // another way to write the same logic, using closure.
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Shortcuts for Panic on Error: unwrap and expect
    // The Result<T, E> type has many helper methods defined on it to do various,
    // more specific tasks. The unwrap method is a shortcut method implemented
    // just like the match expression. If the Result value is the Ok variant,
    // unwrap will return the value inside the Ok. If the Result is the Err variant,
    // unwrap will call the panic!
    let greeting_file = File::open("hello.txt").unwrap();

    // Similarly, the expect method lets us also choose the panic! error message.
    // Using expect instead of unwrap and providing good error messages can convey
    // your intent and make tracking down the source of a panic easier.
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    let result = propagating_errors();
}


// When a function’s implementation calls something that might fail, instead of
// handling the error within the function itself, you can return the error to
// the calling code so that it can decide what to do.
fn propagating_errors() ->  Result<String, io::Error> {
    let result = read_username_from_file();

    // A Shortcut for Propagating Errors: the ? Operator
    let result = read_username_from_file_v2();

    // You can even shorten this code further by chaining method calls immediately
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    // Instead of calling panic!, we use the return keyword to return early out
    // of the function entirely and pass the error value from File::open, now in
    // the pattern variable e, back to the calling code as this function’s error
    // value.

    let mut username = String::new();

    // If we have a file handle in username_file, the function then creates a new
    // String in variable username and calls the read_to_string method on the file
    // handle. The read_to_string method also returns a Result because it might fail,
    // even though File::open succeeded. So we need another match to handle that
    // Result.

    // If read_to_string succeeds, then our function has succeeded, and we return
    // the username from the file that’s now in username wrapped in an Ok.
    // If read_to_string fails, we return the error value.
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Shortcut to Propagating Errors
// If the value of the Result is an Ok, the value inside the Ok will get returned
// from this expression, and the program will continue. If the value is an Err,
// the Err will be returned from the whole function as if we had used the return
// keyword so the error value gets propagated to the calling code.

fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// The error values that have the ? operator called on them go through the from
// function, defined in the From trait in the standard library, which is used to
// convert values from one type into another. When the ? operator calls the from
// function, the error type received is converted into the error type defined in
// the return type of the current function. This is useful when a function returns
// one error type to represent all the ways a function might fail, even if parts
// might fail for many different reasons.


// You can ? on Option in a function that returns an Option.
// If the value is None, the None will be returned early from the function at that point.
// If the value is Some, the value inside the Some is the resulting value of the
// expression and the function continues
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// If text is the empty string, the call to next() will return None, in which case
// we use ? to stop and return None from last_char_of_first_line. If text is not
// the empty string, next will return a Some value containing a string slice of
// the first line in text.

