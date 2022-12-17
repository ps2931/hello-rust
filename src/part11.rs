// Generic Types, Traits and Lifetimes

pub fn run() {

    // Removing code duplication
    // Generics allow us to replace specific types with a placeholder that represents
    // multiple types to remove code duplication.

    // Consider a function largest to find biggest element in a list
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("largest i32 {}", result);

    // Out largest function will work only for i32 types. What is we want to find
    // largest element from a list of chars.

    // We have to write another function, like this:
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("largest char {}", result);

    // If we want one function which should work for i32 and char or any other type
    // which supports Ordering, we have to write a generic function with following
    // signature: fn largest<T>(list: &[T]) -> &T where T: std::cmp::PartialOrd
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("largest number is {}", result);

    let char_list = vec!['s', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);


    // Generics in struct definition
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // Generic in method definition
    // We can implement methods on structs and enums and use generic types in their definitions
     let p = Point { x: 5, y: 10 };
     println!("p.x = {}", p.x());


}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T>(list: &[T]) -> &T
where T: std::cmp::PartialOrd
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// std::cmp::PartialOrd,  is a trait, which is required here because we cannot find
// largest of a type T if we can not order the elements of that type T.

// A generic struct
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// Note that we have to declare T just after impl so we can use T to specify that
// we’re implementing methods on the type Point<T>. By declaring T as a generic
// type after impl, Rust can identify that the type in the angle brackets in Point
// is a generic type rather than a concrete type. We could have chosen a different
// name for this generic parameter than the generic parameter declared in the
// struct definition, but using the same name is conventional.

// We can also specify constraints on generic types when defining methods on the
// type. We could, for example, implement methods only on Point<f32> instances
// rather than on Point<T> instances with any generic type.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// This code means the type Point<f32> will have a distance_from_origin method;
// other instances of Point<T> where T is not of type f32 will not have this
// method defined.

