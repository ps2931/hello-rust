////////////////////////////
// Ownership
////////////////////////////

pub fn run() {
    // Ownership is a set of rules that governs how a Rust program manages memory.
    
    // Stack and Heap
    // Both the stack and heap are parts of memory. Stack is LIFO.
    // Heap is less organized: when you put data on the heap, you 
    // request a certain amount of space. The memory allocator,
    // allocates the required memory and returns address of the location - a pointer.
    
    // Pushing to the stack is faster than allocating on the heap because the allocator 
    // never has to search for a place to store new data; that location is always at the 
    // top of the stack.
    
    // Also accessing data in the heap is slower than accessing data on the stack because 
    // you have to follow a pointer to get there.
    
    // When your code calls a function, the values passed into the function (including, potentially, 
    // pointers to data on the heap) and the function’s local variables get pushed onto the stack. 
    // When the function is over, those values get popped off the stack.
    
    // Keeping track of what parts of code are using what data on the heap, minimizing the amount of 
    // duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space 
    // are all problems that ownership addresses.
    
    // Ownership rules:-
    // 1. Each value in Rust has an owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
    
    variable_scope();    
    strings_and_heap();
    ownership_and_functions();
}

fn variable_scope() {
    let morning_message = "Hello";
    { // evening_message is not valid here.
        let evening_message = "Bye";
    } // evening_message is no longer valid, its scope is over.    
}

fn strings_and_heap() {
    let str_literal = "a_string_literal";
    // String literal is a value hardcoded into your program.
    // String literals are convenient but they are not suitable for every situation in which
    // we want to use text. One reason is that they are immutable. Another is not every string
    // value can be known when we write our code.
    
    // For situation like mentioned above, we have a second string type `String`. This type manages 
    // data allocated on the heap and as such is able to store an amount of text that is unknown to 
    // us at compile time. 
    
    let a_message = String::from("Hello");
    
    // mutable string
    let mut b_message = String::from("Hello");
    b_message.push_str(", World!");
    println!("{}", b_message);
    
    // In order to support a mutable, growable piece of text, we need to allocate an amount of memory
    // on the heap, unknown at compile time, to hold the contents. This means:
    // 1. The memory must be requested from the memory allocator at runtime.
    // 2. We need a way of returning this memory to the allocator when we’re done with our `String`.
    
    // The second is where programming languages such as Java use garbage collector.
    // Rust takes a different approach: the memory is automatically returned once the 
    // variable that owns it goes out of scope.
    
    // When a variable goes out of scope, Rust calls a special function for us. This function is called drop.
    // Rust calls drop automatically at the closing curly bracket.
    
    /////////////////////////////////////////
    // Ways Variables and Data Interact: Move
    /////////////////////////////////////////
    let x = 5;
    let y = x; // Rust copies value of x into y.
    
    // With strings the case is bit different.
    let s1 = String::from("Hello");
    let s2 = s1;
    
    // s1 is reference to a memory location in heap, which container 'Hello'.
    // `let s2=s1` cause reference to be copied into s2 instead of heap content.
    
    // We know that when a variable goes out of scopeRust automatically calls the `drop` function.
    // But s1 and s2 both refers to same memory location. This is a problem: when s2 and s1 go out 
    // of scope, they will both try to free the same memory. This is known as a double free error.
    
    // To ensure memory safety, after the line `let s2 = s1`, Rust considers s1 as no longer valid.
    // Therefore, Rust doesn't need to free anything when s1 goes out of scope.
    
    // This can be prooved by the fact that, s1 is not acceble once you assigned it to s2.
    
    // println!("{}", s1); Error: borrow of moved value
    
    //////////////////////////////////////////
    // Ways Variables and Data Interact: Clone
    //////////////////////////////////////////
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1={}, s2={}", s1,s2);
    // This works just fine, s1 and s2 both are accessbile since 
    // we copied the heap data and assigned its pointer to s2.
    
    // NOTE: Types such as integers that have a known size at compile time are stored 
    // entirely on the stack, so copies of the actual values are quick to make.
    
    // Rust has a special annotation called the Copy trait that we can place on types that are 
    // stored on the stack. If a type implements the Copy trait, variables that use it do not move, 
    // but rather are trivially copied, making them still valid after assignment to another variable.
    
    // Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented 
    // the Drop trait.    
}

fn ownership_and_functions() {
    // The mecahnics of passing a value to a function are similar to those when assigning 
    // a value to a variable.
    
    let message = String::from("Hello");
    takes_ownership(message); // message value moved into the function and so
                            // no longer valid here

    let x = 54;
    makes_copy(x); // a copy of x is passed to the function.
}

fn takes_ownership(message: String) {
    println!("{}", message);
}

fn makes_copy(n: i32) {
    println!("{}", n);
}

fn return_values_and_scope() {
    // Returning values can also transfer ownership.
    let s1 = gives_ownership();   
    let s2 = String::from("Hello");    
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(s: String) -> String {
    s
}

// While this works, taking ownership and then returning ownership with every function is a bit tedious.
// What if we want to let a function use a value but not take ownership.

// References are to the rescue!!