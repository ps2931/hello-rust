///////////////////////////////
// Common Programming Concepts
///////////////////////////////

pub fn run() {
    variables_and_mutability();
    constants();
    shadowing();
    data_types();
    functions();
    control_flow();
}

fn variables_and_mutability() {
    println!("=========Inside Variables and Mutability=========");
    let mut x = 5;
    println!("x is {x}");
    x = 6; //possible because x is defined as mut
    println!("x is {x}");
}

fn constants() {
    println!("=========Inside Constants=========");
    // Constants are values that are bound to a name and are not allowed to 
    // change. The difference between immutable variables and constants are:
    // 1. You cannot use mut keyword wit constants.
    // 2. The type of value must be annotated.
    // 3. Constants can be declared in any scope including global scope.
    // 4. Constants may be set only to a constant expression, not the result of a value
    // that could only be computed at runtime.

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}

fn shadowing() {
    let x = 5;
    let x = x+1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}"); // 12
    }
    println!("The value of x is {x}"); // 6
    
    // We can even change the type of variable
    let x = 6.7;
    println!("The value of x is now {x}");
}

fn data_types() {
    println!("=========Inside Data Types=========");
    // Rust being a statically typed language, it must know the types of all
    // variables at compile time.
    
    // Integers
    // i8, i16, i32, i64, i128, isize
    // u8. u16, u32, u64, u128, usize
    println!("{}", 0xff); // 255
    println!("{}", 0xff * 0o77); // 16065
    println!("{}", 0b10111_011); // 187
    
    // Floating points
    let x = 2.0; // f64 - default
    let y: f32 = 3.0;
    
    // Numeric operation
    let sum = 5+10; 
    let subtraction = 54.2-16.6;
    let multiply = 4 * 30;
    let quotient = 56.7/32.2; // 1.7608695652173911
    let remainder = 12%5; // 2
    
    // Boolean type
    let t = true;
    let f = false;
    
    // Character type
    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value.
    let c = 'z';
    let z: char = 'ℤ';
    
    // Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive. 
    // However, a “character” isn’t really a concept in Unicode, so your human intuition 
    // for what a “character” is may not match up with what a char is in Rust.
    
    // Compound type
    
    // Tuple type
    let tuple:(i32, f64, u8) = (500, 6.4, 1);
    // Pattern matching in tuple
    let tup = (500, 6.4, 1);
    println!("x is {}", tup.0);

    let (x, y, z) = tuple; // destructuring
    println!("y is {}", y);
    
    // Array type
    let a1 = [1, 2, 3, 4, 5];
    let a2: [i32; 5] = [3, 5, 7, 11, 13]; // explicit type definition
    let a3 = [3; 5]; // 5 elements; all set to 3
    let first = a1[0];
    let second = a1[1];
}

fn functions() {
    println!("=========Inside Functions=========");
    another_function(5);
    multiple_parameters(5, 'Z');
    let five = func_returning_value(4);
    println!("Returned value is {five}");
}

fn another_function(x: i32) {
    println!("x is {x}");
}

fn multiple_parameters(value: i32, c: char) {
    println!("first parameter: {value} , second parameter: {c}");
}

fn func_returning_value(i: i32) -> i32{
    i+1 // return value as there is no semi-colon
        // if you want semiclon use return keyword
}

fn statements_and_expression() {
    let y = 6; // statement
    
    // Expressions evaluate to a value 
    let y = {
        let x = 3;
        x+1 // notice no semi-colon
    };            
}

fn control_flow() {
    println!("=========Inside Control Flow=========");
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4, 3");
    }
    
    // if an let
    // both arm must have same return type
    let number = if 2>1 {5} else {6}; 
    println!("value is {number}");
    
    // loop
    let mut x = 1;
    loop {
        println!("you are in loop");
        if x == 3 {break}
        x += 1;
    }
    
    // returning values from loops
    let mut counter = 0;
    let result = loop{
      counter += 1;  
      if counter == 5 {
          break counter * 2;
      }
    };
    println!("result is {result}"); // 10
    
    // Loop labels to Disabmbiguate between multiple loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count {count}"); // 2
    
    // while loop
    let mut number = 3;
    while number !=0 {
        println!("{number}");
        number -= 1;
    }
    println!("while its done :)");
    
    // loop throuhg an array
    let arr = [10,20,30,40,50];
    let mut i = 0;
    while i < 5 {
        println!("value is {}", arr[i]);
        i += 1;
    }
    
    // more safer way to loop through array
    println!("===safe loop through array===");
    let arr = [10,20,30,40,50];
    for element in arr {
        println!("value is {element}");
    }
    
    // range based for loop
    println!("==range based for loop==");
    for number in (1..4).rev() {
        print!("{number}! ");
    }
    println!();
    // start and end inclusive
    for number in (1..=4).rev() {
        print!("{number}! "); // 4 3 2 1
    }
    println!();
}