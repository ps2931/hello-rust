pub fn run() {

    // An iterator is a value that produces a sequence of values, typically for a
    // loop to operate on. Rust's standrd library provides iterators that traverse
    // vectors, strings, hash tables, and other collections, but also iterators to
    // produce lines of text from an input stream, connections arriving at a network
    // server, values received from other threads over a communication channel,and so on.

    // Consider the following function, which returns the sum of the first n positive
    // integers (nth triangle).

    fn traingle_v1(n: i32) -> i32 {
        let mut sum = 0;
        for i in 1..n+1 {
            sum += i;
        }
        sum
    }

    // The expression 1..n+1 is a Range<i32> value. A Range<i32> is an iterator that
    // produces the integers from its start value to its end value (exclusive).

    // You can also use iterator's fold method, like this
    fn traingle_v2(n: i32) -> i32 {
        (1..n+1).fold(0, |sum, item| sum+item)
    }

    // The Iterator and IntoIterator Traits:
    // An iterator is any value that implements the std::iter::IntoIterator trait.
    /* `trait Iterator {
            type Item;
            fn next(&mut self) -> Option<Self::Item>;
        }`

    If there is a natural way to iterate over some type, it can implement std::iter::IntoIterator,
    whose into_iter method takes a value and returns an iterator over it.

        pub trait IntoIterator {
            type Item;
            type IntoIter: Iterator<Item = Self::Item>;
            fn into_iter(self) -> Self::IntoIter;
        }

    IntoIter is the type of iterator value itself and Item is the type of value
    it produces.

    */

    // for loop to iterator over a vector
    let v = vec!["apple", "guava", "banana", "plum"];
    for element in & v {
        println!("{}", element);
    }

    // Under the hood, every for loop is just shorthand for calls to IntoIterator
    // and Iterator methods.
    let mut iterator = (&v).into_iter();
    while let Some(element) = iterator.next() {
        println!("{}", element);
    }


    // The for loop uses IntoIterator::into_iter to convert its operand &v into an
    // iterator, and then calls Iterator::next repeatedly.


    //////////////////////
    // Creating Iterators
    //////////////////////

    // iter and iter_mut methods
    // Most collection types provide iter and iter_mut methods that return the natural
    // iterators over the type, producing a shared or mutable reference to each item.
    // Slices like &[T] and &str have iter and iter_mut methods too.
    let v = vec![4, 20, 12, 8, 6];
    let mut iterator = v.iter();
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), Some(&20));
    assert_eq!(iterator.next(), Some(&12));
    assert_eq!(iterator.next(), Some(&8));
    assert_eq!(iterator.next(), Some(&6));
    assert_eq!(iterator.next(), None);

    // IntoIterator Implementations
    // When a type implements IntoIterator, you can call its into_iter method yourself:
    use std::collections::BTreeSet;
    let mut favorites = BTreeSet::new();
    favorites.insert("Lucy in the dark night".to_string());
    favorites.insert("Men in Black".to_string());

    let mut it = favorites.into_iter();
    assert_eq!(it.next(), Some("Lucy in the dark night".to_string()));
    assert_eq!(it.next(), Some("Men in Black".to_string()));


    // Most collections actually provid several Implementations of IntoIterator, for
    // shared references, mutable references and moves.

    // 1. Given a shared reference to the collection, into_iter returns an iterator
    // that produces shared references to its items. For example, (&favorite).into_iter()
    // would return an iterator whose Item type is &String.

    // 2. Given a mutable reference to the collection, into_iter returns an iterator
    // that produces mutable references to the items. For example, if vector is some
    // Vec<String>, the call (&mut vector).into_iter() returns an iterator whose
    // Item type is `&mut String`.

    // 3. When passed the collection by value, into_iter returns an iterator that takes
    // ownership of the collection and return items by value. The item ownership moves
    // from the collection to the consumer.

    // Since a for loop applies IntoIterator::into_iter to its operand, these three
    // implementations are what create the following idioms:
    // for element in &collection {..}
    // for element in &mut collection {..}
    // for element in collection {..}

    // Not every type provide all three implementations. For example, HashSet, BTreeSet
    // and BinaryHeap don't implement IntoIterator on mutable references, since modifying
    // their elements would probably violate the type's invariants.

    // Slices implements two of three IntoIterator variants: since they don't own
    // their elements, there is no "by value" case.

    // The first two IntoIterator variants, for shared and mutable references are
    // equivalent to calling iter or iter_mut on the referent.

    // IntoIterator can also be useful in generic code: you can use a bound like
    // T: IntoIterator to restrict the type variable T to types that can be iterated
    // over. Or you can write T: IntoIterator<Item=U> to further require the iteration
    // to produce a particular type U.

    // drain methods
    // Many collection types provide a drain method that takes a mutable reference
    // to the collection and returns an iterator that passes ownership of each element
    // to the consumer. drain merely borrows a mutable reference to the collection,
    // and when the iterator is dropped, it removes any remaining elements from the
    // collection and leaves it empty. Unlike the into_iter() method, which takes
    // the collection by value and consumes it.

    use std::iter::FromIterator;

    let mut outer = "Earth".to_string();
    let inner = String::from_iter(outer.drain(1..4));

    assert_eq!(outer, "Eh");
    assert_eq!(inner, "art");

    // If you do need to drain the entire sequence, use the full range, .., as argument.

    ////////////////////
    // Iterator Adapters
    ////////////////////

    // Adapters consume one iterator and build a new one with useful behaviors.

    // map and filter
    // The Iterator trait's map adapter lets you transform an iterator by applying
    // a closure to its items. The filter adapter lets you filter out items from an
    // iterator, using a closure to decide which to keep and which to drop.

    // remove leading and trailing spaces from each line
    let text = " apple  \n  mango\nbanana   \nplum".to_string();
    let v: Vec<&str> = text.lines()
        .map(str::trim)
        .collect();

    assert_eq!(v, ["apple", "mango", "banana", "plum"]);

    // The text.lines() call returns an iterator that produces the string's lines.
    // Calling map on that iterator returns a second iterator that applies str::trim
    // to each line. Finally, collect gathers those items into a vector.

    // The iterator map returns is itself a candidate for further adaption. Suppose
    // you want to exclude banana from the result:
    let v: Vec<&str> = text.lines()
        .map(str::trim)
        .filter(|s| *s != "banana")
        .collect();

    assert_eq!(v, ["apple", "mango", "plum"]);

    // A map iterator passes each item to its closure by value, and in turn, passes
    // along ownership of the closure's result to its consumer. A filter iterator
    // passes each item to its closure by shared reference, retaining ownership
    // in case the item is selected to be passed on to its consumer. This is why
    // we dereferenced s to compare it to "banana": the filter iterator's item type
    // is &str, so the type of the closure's argument s is &&str.

    // Note that simple calling an adapter on an iterator doesn't consume any items;
    // it just returns a new iterator, ready to produce its own items by drawing from
    // the first iterator as needed. In a chain of adapters the only way to make any
    // work actually get done is to call next on the final iterator.

    // Hence, map and filter just return new iterators that would map or filter if asked.
    // No work takes place until collect starts calling next on the filter iterator.

    // filter_map and flat_map
    // The map adapter is fine in situations where each incoming item produces one outgoing
    // item. But what if you want to delete certain items from the iteration instead of
    // processing them, or replace single item with zero or more items? The filter_map
    // and flat_map adapters grant you this flexibility.

    // The filter_map adapter is similar to map exept that it lets its closure either
    // transform the item into a new item or drop the item from the iteration.
    //

}
