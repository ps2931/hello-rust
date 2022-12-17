//////////////////////////////////
// Referencing and Borrowing
//////////////////////////////////

pub fn run() {
    // A reference is like a pointer in that it's an address we can follow to access 
    // the data stored at that address; that data is owned by some other variable.
    
    passing_ref_to_fn();
    mutable_references();      
    reference_restrictions();
    slice_type();
}

fn passing_ref_to_fn() {
    let s1 = String::from("Hello");
    let lenght = calculate_length(&s1); // ref passing or borrowing
    println!("Lenght of '{}' is {}.", s1, lenght);
}

// The scope in which the variable s is valid is the same as any function parameter’s scope, 
// but the value pointed to by the reference is not dropped when s stops being used because s 
// doesn’t have ownership.
fn calculate_length(s: &String) -> usize {
    // references are immutable by default
    // ERROR: s.push_str(", World");
    s.len()
}

fn mutable_references() {
    let mut s = String::from("Hello");
    change_string(&mut s); // passing mutable reference
    println!("String after change: {}", s);
}

fn change_string(s: &mut String) {
    s.push_str(", World");
}

// If you have mutable reference to a value, you can have no other references to that value.
// The benefit of this restriction is that Rust can prevent data races at compile time.
fn reference_restrictions() {
    let mut s = String::from("Hello");
    let r1 = &mut s;    
    // ERROR `let r2 = &mut s;`    
    // cannot borrow `s` as mutable more than once at a time
    println!("{}", r1);
    
    // As always, you can use curly brackets to create a new scope, allowing for 
    // multiple mutable references, just not simulataneuos ones:
    let mut s1 = String::from("hello");
    {
        let r1 = &mut s1;
    }
    let r2 = &mut s1;
    
    // You cann;t even mutable reference to a variable, which is alread have immutable reference
    let mut s2 = String::from("Hello");
    let r1 = &s2;
    let r2 = &s2;
    // ERROR: let r3 = &mut s2;
    // cannot borrow `s2` as mutable because it is also borrowed as immutable
    println!("{}, {}", r1, r2);
    
    // From code above it's also clear that multiple immutable references are allowed 
    // because no one has ability to change the data, and hence no race condition.
    
    let r3 = &mut s2;
    println!("{}", r3);
    // The scopes of the immutable references r1 and r2 end after the println! where they are last used, 
    // which is before the mutable reference r3 is created. These scopes don’t overlap, so this code is 
    // allowed.
}


// The code below is not allowed as we are returning a local variable s from dangling() function.
// The variable will be out of scope or deallocated after function retruns. That means ref_to_nothing
// will be pointing to invalid String.

// fn dangling_pointer() {
//     let ref_to_nothing = dangling();
//     println!("{:?}", ref_to_nothing);
// }

// fn dangling() -> &String {
//     let s = String::from("Hello");
//     &s
// }

fn slice_type() {
    // Slice lets you reference a contiguous sequence of elements in a collection rather than the whole
    // collection. A slice is a kind of reference, so it does not have ownership.
    let s = String::from("Hello World");
    
    let hello = &s[0..5];
    let world = &s[6..11];
    
    // The variable hello is a reference to a portion of the String. Internally, the slice data structure
    // stores the starting position and the length of the slice, which corresponds to (ending_index - starting_index).
    
    // same as above
    let hello_1 = &s[..5];
    let world_1 = &s[6..];
    
    // slice which is equal to the entire string
    let hello_world = &s[..];
    
    // write a function to get first word of a sentence
    let s = String::from("Goodbye World");
    let w = first_word(&s);
    println!("{}", w);
    
    // This code will not compile
    let mut s2 = String::from("Make a hay while the sun shine");
    let word = first_word(&s2);
    // s2.clear(); 
    // cannot borrow `s2` as mutable because it is also borrowed as immutable
    // println!("First word is {}", word); // borrowed immutably here
    
    // Other slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);        
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Above method can be written using string slices only.
// fn first_word(s: &str) -> &str { ..}
// It allows us to use the same function on both &String values and &str.

