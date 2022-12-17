/////////////////////////
// Rust Collections
/////////////////////////

/*
   Rust’s standard library includes a number of very useful data structures called collections.

   Rust’s collections can be grouped into four major categories:
   Sequences: Vec, VecDeque, LinkedList
   Maps: HashMap, BTreeMap
   Sets: HashSet, BTreeSet
   Misc: BinaryHeap

*/

pub fn run() {
    vectors();
    enums();
    strings();
    hashmaps();
}

fn vectors() {
    // creating a new empty vector
    let v: Vec<i32> = Vec::new();
    // Type annotation is require as our vector is empty and Rust cannot deduce the type.

    // creating a vector with elements
    let v = vec![1, 2, 3];

    // updating a vector - use mut
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    // Reading elemenst of vector
    let second: &i32 = &v[1]; // indexing starts with 0
    println!("second: {}", second);

    // another way to read vector elements
    let third: Option<&i32> = v.get(2);
    match third {
        Some(n) => println!("third: {}", n),
        None => println!("no thrid elements"),
    }

    // This is not allowed
    // let mut v = vec![1,2,3,4,5];
    // let frist = &v[0]; // immutable borrow
    // v.push(10); // mutable borrow
    // println!("first: {}", first); // immutable borrow used here

    // Iterating over a vector
    let v = vec![3, 5, 7, 11];
    for e in v {
        println!("{}", e); // e is immutable
    }

    // Iterate using mutable reference
    let mut v = vec![3, 5, 7, 11];
    for e in &mut v {
        *e += 10;
    }
    println!("vector: {:?}", v); // [13, 15, 17, 21]
}

// Enums
// Vectors can only store valuse that are the same type.
// To store values of different type enum/struct is required.

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click {x: i64, y: i64},
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn enums() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

fn strings() {
    // Rust only has one type of string in the core language, which is string slice str.
    // The String type provided by Rust's standard library is growable, mutable, owned
    // UTF-8 encoded string type.

    // Creating a new empty String
    let mut s = String::new();

    // Create a string with data
    let data = "initial contents"; // &str
    let s = data.to_string(); // String

    // one more way
    let s = String::from("initial contents");

    // Strings are UTF-8 encoded
    let hello = String::from("Здравствуйте");

    // Updating a string
    let mut s = String::from("foo");
    s.push_str("bar"); // foobar

    // The push_str method takes a string slice because we dont't necessarily
    // want to take ownership of the parameter.
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2: {}", s2); // owneeship is still with s2

    // add a single character
    let mut s = String::from("LO");
    s.push('L');
    println!("{}", s); //LOL

    // String concatenation
    let s1 = String::from("Hello ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is moved here and can no longer be used

    // The + operator uses the `add` method, whose signature is:
    // fn add(self, s: &str) -> String {}
    // Note that first parameter takes owenrship of self, as self does not have &,
    // and the second parameter is reference.

    // For more complicated string combining, we can instead use the format! macro:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1,s2,s3);

    // Indexing into Strings
    // Rust does not support string indexing. The reason is internal representation of UTF-8 strings.
    // A String is a wrapper over Vec<u8>.
    let en_hello = String::from("Hello"); // 5 bytes, lenght is 5
    let ru_hello = String::from("Здравствуйте"); // 24 bytes, length is 12

    // ru_hello string takes 24 bytes memory because each unicode scalar value in that
    // string takes 2 bytes of storage. Therefore, an index into the string's bytes will
    // not always correlate to a valid Unicode scalar value.
    let hello = "Здравствуйте";
    // Suppose Rust allows us to write this.
    // let first_char = &hello[0];
    // The value of first_char will not be 3, the first letter.

    // If you really need to use indices to create slices, be more specific.
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // &str

    // Here, s will be a &str that contains the first 4 bytes of the string.
    // Earlier, we mentioned that each of these characters was 2 bytes, which means
    // s will be Зд.

    // ERROR &hello[0..1]; // this is not a char

    // There are three different ways to look at strings from Rust's perspective.
    // 1. as bytes 2. scalar values 3. grapheme clusters (letters)

    // 1. String as bytes
    let sparkle_heart_vec: Vec<u8> = vec![240, 159, 146, 150];
    let sparkle_heart_str = String::from_utf8(sparkle_heart_vec).unwrap();

    assert_eq!("💖", sparkle_heart_str);

    // Iterating over Strings
    // For individual Unicode scalar values, use the chars method.
    for c in "Зд".chars() {
        println!("{}", c);
    }

    // The byte method returns each raw byte
    for b in "Зд".bytes() {
        println!("{}", b);
    }

}

fn hashmaps() {
    // Creating a new hash map
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Access HashMap values
    let score = scores.get("Blue").copied().unwrap_or(0);

    // The get method returns an Option<&V>; if there’s no value for that key
    // in the hash map, get will return None. This program handles the Option
    // by calling copied to get an Option<i32> rather than an Option<&i32>, then
    // unwrap_or to set score to zero if scores doesn't have an entry for the key.

    // Iterate over each key/value pair
    for (key, value) in &scores {
        println!("{} \t {}", key, value);
    }

    // For types that implement the Copy trait, like i32, the values are copied
    // into the hash map. For owned values like String, the values will be moved
    // and the hash map will be the owner of those values.

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point

    // Updating a HashMap
    // If a key is already present in the HashMap, you need to decide whether you
    // want to replace the old value or you want to keep the old value, only adding
    // the new value if the key doesn't exist.

    // Overwriting a value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    // after above line, scores will have 25

    // Add only if key is not present
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(60);
    println!("{:?}", scores); // {"Blue": 10, "Yellow": 60}

    // Updating a value based on the old value
    let text = "to be or not to be or be";
    let mut word_count_map = HashMap::new();

    for word in text.split_whitespace() {
        let count: &mut i32 = word_count_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", word_count_map);
    // {"to": 2, "not": 1, "be": 3, "or": 2}

    // The split_whitespace method returns an iterator over sub-slices, separated
    // by whitespace, of the value in text. The or_insert method returns a mutable
    // reference (&mut V) to the value for the specified key.


}
