// Functional Language Features

// Programming in a functional style often includes using functions as values by passing them in
// arguments, returning them from other functions, assigning them to variables for later execution,
// and so forth.

struct City {
    name: String,
    population: i64
}

/*

A sort function to order struct type
fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| -city.population);
}

It returns a -ve number because sort arranges numbers in increasing order.

Capturing Variables:
A closure can use data that belongs to an enclosing function.

fn sort_by_statistics(cities: &mut Vec<City>, stat: Statistics) {
    cities.sort_by_key(|city| -city.get_statistics(stat));
}

Normally when a function returns, all its variables and arguments go out of scope.
But Rust must keep stat around somehow, since the closure uses it. How?? - Rust ensures
safety by using lifetimes instead of GC like Java.

Closures that Borrow:
fn sort_by_statistics(cities: &mut Vec<City>, stat: Statistics) {
    cities.sort_by_key(|city| -city.get_statistics(stat));
}

Here, when Rust creates the closure, it automatically borrows a reference to stat.
It stands to reason: the closure refers to stat, so it must have a reference to it.

Closures that Steal:
use std::thread;

fn start_sorting_thread(mut cities: Vec<City>, stat: Statistics) -> thread::JoinHandle<Vec<City>>
{
    let key_fn = |city: &City| -> i64 { -city.get_statistics(stat) };

    thread::spawn(|| {
        cities.sort_by_key(key_fn);
        cities;
    })
}

The thread::spawn takes a closure and calls it in a new system thread. The new thread
runs in parallel with the caller. When the closure returns, the new thread exits.
The closure's return value is sent back to the calling thread as a JoinHandle value.

Again, the closure key_fn contains a reference to stat. But this time, Rust can't
gurantee that the reference is used safely. Rust therefore rejects this program.

The solution is to tell Rust to move cities and stat into the closure that use them
instead of borrowing references to them.

fn start_sorting_thread(mut cities: Vec<City>, stat: Statistics) -> thread::JoinHandle<Vec<City>>
{
    let key_fn = move |city: &City| -> i64 { -city.get_statistics(stat) };

    thread::spawn(move || {
        cities.sort_by_key(key_fn);
        cities
    })
}

We added the move keyword before eash of the two closures. The move keyword tells
Rust that a closure doesn't borrow the variables it uses: it steals them.

The first closure, key_fn takes ownership of stat. Then the second closure takes
ownership of both cities and key_fn.

Rust thus offers two ways for closures to get data from enclosing scopes:
moves and borrowing.

Note:
    1. Just as everywhere else in the language, if a closure would move a value
    of copy-able type like i32, it copies the value instead. So if Statistics happened
    to be a copyable type, we could keep using stat even after creating move closure
    that uses it.

    2. Values of non-copyable types like Vec<City>, really are moved.

The strict rules Rust follows gives us thread safety. It's because the vector is
moved, rather than being shared across threads, that we know the old thread won't
free the vector while the new thread modifying it.

Function and Closure Types:

fn city_population_descending(city: &City) -> i64 {
    -city.population
}

This function takes one argument (a &City) and returns an i64. It has the type
fn(&City) -> i64.

let my_key_fn: fn(&City) -> i64 =
    if user.perfs.by_population {
        city_population_descending
    } else {
        city_monster_attack_rist_descending
    }

cities.sort_by_key(my_key_fn);

A function can take another function as an argument.

fn count_selected_cities(cities: &Vec<City>,
                        test_fn: fn(&City) -> bool) -> usize
{
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    }
    count
}

An example of a test function. Note that the type of this
function is `fn(&City) -> bool`, the same as the `test_fn`
argument to `count_selected_cities`
fn has_monster_attacks(city: &City) -> bool {
    city.monster_attack_risk > 0.0
}

// How many cities are at risk for monster attack?
let n = count_selected_cities(&my_cities, has_monster_attacks);


But surprisingly, closures do not have same type as functions:

let limit = preferences.acceptable_monster_risk();
let n = count_selected_cities(
        &my_cities,
        |city| city.monster_attack_risk > limit); // error: type mismatch

To support closures, we must change the type signature to this function.

fn count_selected_cities<F>(cities: &Vec<City>, test_fn: F) -> usize
where F: Fn(&City) -> bool {..}

This new version is generic. It takes a test_fn of any type F as long as F
implements the special trait Fn(&City) -> bool. This trait is automatically
implemented by all functions and closures that take a single &City as an argument
and return a Boolean value.

fn(&City) -> bool  // fn type (functions only)
Fn(&City) -> bool  // Fn trait (functions and closures both)

The new version of count_selected_cities accpets either a function or a closure.

count_selected_cities(
    &my_cities,
    has_monster_attacks); // ok
    )

count_selected_cities(
    &my_cities,
    |city| city.monster_attack_risk > limit); // also ok
*/


