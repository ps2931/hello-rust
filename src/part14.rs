// Iterators

/*
   An iterator is a value that produce a sequence of value.
*/
pub fn run() {
    let sum = triangle_v1(10);
    println!("{}", sum); // 55

    // In triangle function 1..n+1 is a Range<i32> value. A Range<i32> is an iterator
    // that produces the integers from its start value (inclusive) to end value (exclusive).

    // Iterators also have a fold method, which can be used instead of range().
    let sum = triangle_v2(10);
    println!("{}", sum); // 55

    /*
    The Iterator and IntoIterator Traits
    An iterator is any value that impelments the std::iter::Iterator trait.
    If there is a natural way to iterate over some type, it can implement
    std::iter::IntoIterator, whose into_iter method takes a value and returns
    and iterate over it.

    trait IntoIterator where Self::IntoIterator::Item == Self::Item {
        type Item;
        type IntoIter: Iterator;
        fn into_iter(self) -> Self::IntoIter;
    }

    IntoIter is the type of the iterator value itself, and Item is the type of value
    it produces.
    */

    // Rust's for loop brings all these parts together nicely. To iterate over a vector's
    // elements, you can write:
    println!("for loop");
    let v = vec!["antimony", "arsenic", "aluminium", "selenium"];
    for element in &v {
        println!("{}", element);
    }

    // Under the hood, every for loop is just shorthand for calls to IntoIterator
    // and Iterator methods.
    println!("Iterator");
    let mut iterator = (&v).into_iter();
    while let Some(element) = iterator.next() {
        println!("{}", element);
    }

    // Creating Iterators
    // iter and iter_mut methods:
    // Most collection types provide iter and iter_mut method that return the natural
    // iterators over the type,producing a shared or mutable reference to each item.
    // These methods are the most common way to get an iterator, if you're not going
    // to let a for loop take care of it for you:
    let v = vec![4,20,12,8,6];
    let mut iterator = v.iter();
    assert_eq!(iterator.next(), Some(&4));

    // Each iterator's item type is &i32: each call to next produces a reference to the next
    // element, until we reach the end of the vector.

    // Each type is free to implement iter and iter_mut in whatever way makes the most
    // sense for its purpose. The iter method on std::path::Path returns an iterator
    // that produces one path componenet at a time:

    use std::ffi::OsStr;
    use std::path::Path;

    let path = Path::new("/home/pankaj/Downloads/drive.zip");
    let mut iterator = path.iter();

    let item:Option<&OsStr> = iterator.next();
    assert_eq!(item, Some(OsStr::new("/")));

    let item:Option<&OsStr> = iterator.next();
    assert_eq!(item, Some(OsStr::new("home")));

    let item:Option<&OsStr> = iterator.next();
    assert_eq!(item, Some(OsStr::new("pankaj")));


    // IntoIterator Implementations
    use std::collections::BTreeSet;
    let mut favorites = BTreeSet::<String>::new();
    favorites.insert("Lucky in the Sky with Diamonds".to_string());
    favorites.insert("To be or not to be".to_string());

    let mut it = favorites.into_iter();
    assert_eq!(it.next(), Some("Lucky in the Sky with Diamonds".to_string()));
    assert_eq!(it.next(), Some("To be or not to be".to_string()));

    // Most collections actually provide several Implementations of IntoIterator, for
    // shared references, mutable references, and moves.

    // Given a shared reference to the collection, into_iter returns an iterator that
    // produces shared references to its items i.e (&favorites).into_iter() would
    // return an iterator whose Item type is &String.

    // Given a mutable reference to the collection, into_iter returns an iterator
    // that produces mutable references to the items i.e for Vec<String>, (&mut vector).into_iter()
    // returns an iterator whose Item type is &mut String.

    // When passed the collection by value, into_iter returns an iterator that takes
    // ownership of the collection and returns items by value. The item's ownership
    // moves from the collection to the consumer, and the original collection is
    // consumed in the process.

    // The three idioms defined above can be implemented in for loop as well:
    // for element in &collection {..}
    // for element in &mut collection {..}
    // for element in collection {..}

    // Not every type provides all three implementations. For e.g. HashSet, BTreeSet
    // and BinaryHeap don't implement IntoIterator on mutable references, since modifying
    // their elements would probably violate the type's invariants: the modified value
    // might have different hash value, or be ordered differently with respect to its
    // neighbors, so modifying it would leave it incorrectly placed.

    // Other types do support mutation, but only partially. For e.g HashMap and BTreeMap
    // produce mutable reference to their entries' values, but only shared references
    // to their keys, for similar reasons.

    // Slices implement two of the three IntoIterator variants; since they don't own their
    // elements, there is no "by value" case.

    // You might have noticed that the first two IntoIterator variants, for shared and
    // mutable references are equivalent to calling iter or iter_mut on the referent. Why?
    // IntoIterator is what makes for loop work, so its necessary. But when you're not
    // using a for loop, favorites.iter() is clearer than (&favorites).into_iter().
    // So iter and item_mut are still valuable.

    // IntoIterator can also be useful in generic code: you can use it as a bound
    // like T: IntoIterator to restrict the type variable T to types that can be
    // iterated over. Or you can write T: IntoIterator<Item=U> to further require
    // the iteration produce a particular type U. Example:
    use std::fmt::Debug;
    fn dump<T, U>(t: T)
        where T: IntoIterator<Item=U>, U:Debug
        {
            for u in t {
                println!("{:?}", u);
            }
        }

}

fn triangle_v1(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..n+1 {
        sum += i;
    }
    sum
}

fn triangle_v2(n: i32) -> i32 {
    (1..n+1).fold(0, |sum, item| sum+item)
    // Starting with 0 as the running total, fold takes each value that 1..n+1 produces
    // and applies the closure |sum, item| sum + item to the running total and value.
    // The closures return value is taken as the new running total.
}
