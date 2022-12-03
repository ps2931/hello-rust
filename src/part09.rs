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

fn strings() {}
