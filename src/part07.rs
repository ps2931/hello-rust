//////////////////////////////////
// Enums and Pattern Matching
//////////////////////////////////

// Enumeration allow you to create a new type that can havea value of several
// tagged elements using the enum keyword.

enum Species {
    Crab,
    Octopus,
    Fish,
    Clam,
}

struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon : String,
}

pub fn run() {
    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("Claw"),
    };

    // match helps ensure exhaustive handling of all possible enum values making
    // it a powerful tool in ensuring quality code.
    match ferris.species {
        Species::Crab => println!("{} is a crab", ferris.name),
        Species::Octopus => println!("{} is a octopus", ferris.name),
        Species::Fish => println!("{} is a fish", ferris.name),
        Species::Clam => println!("{} is a clam", ferris.name),
    }

    ////////////////////////
    // Enumeration with Data
    ////////////////////////
    // enum elements can also have one or more data types allowing them to behave like union from C.
    // When an enum is pattern matched using match, you can bind a variable name to each data value.

    // An enum data value will have a memory size equal to its largest elements.
    // In addition to element data types, each element also has a numeric value that represents which tag it is.

    // Rust' enum is also known as tagget union.
    // The combining of types to make a new type is why we say Rust has algebraic types.
    enum_with_data();

    // The Option<T> enum is so useful that it’s even included in the prelude; you
    // don’t need to bring it into scope explicitly. Its variants are also included
    // in the prelude: you can use Some and None directly without the Option:: prefix
    // The Option<T> enum is still just a regular enum, and Some(T) and None are still
    // variants of type Option<T>.
    optional_value();

    // if-let
    // For some use cases, `match` is awkward. E.g
    let optional = Some(7);
    match optional {
        Some(i) => {
            println!("Optional value is {:?}", i);
        }
        _ => {},
    };

    // if-let is cleaner for this use case and in addition allows various failure
    // options to be specified.
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if-let` construct reads: if `let` destructures `number` into `Some(i)`
    // evaluate the block, else do nothing.
    if let Some(i) = number {
        println!("Matched {}", i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {}", i);
    } else {
        // Destructuring failed, change the failure case.
        println!("Didn't match a number.");
    }

    // Provide an altered failing condition
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {}", i);
    } else if i_like_letters {
        println!("Didn't match a number");
    } else {
        println!("I don't like letters.");
    }

}

enum PoisonType { Acidic, Painful, Lethal }
enum Size { Big, Small }
enum Weapon {
    Claw(i32, Size),
    Poison(PoisonType),
    None
}

struct SeaCreaturesV2 {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: Weapon,
}

fn enum_with_data() {
    let ferris = SeaCreaturesV2 {
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(2, Size::Small),
    };

    match ferris.species {
        Species::Crab => {
            match ferris.weapon {
                Weapon::Claw(num_claws,size) => {
                    let size_description = match size {
                        Size::Big => "big",
                        Size::Small => "small"
                    };
                    println!("ferris is a crab with {} {} claws", num_claws, size_description)
                },
                _ => println!("ferris is a crab with some other weapon")
            }
        },
        _ => println!("ferris is some other animal"),
    }
}

// The Option Enum
//  enum Option<T> {
//      None,
//      Some(T),
//  }

fn optional_value() {
    let a_number = Some(5);
    let a_char = Some('z');
    let absent_number: Option<i32> = None;

    // The type of a_number is Option<i32>. The type of a_char is Option<char>,
    // which is a different type. Rust can infer these types because we’ve specified a
    // value inside the Some variant. For absent_number, Rust requires us to annotate
    // the overall Option type: the compiler can’t infer the type that the corresponding
    // Some variant will hold by looking only at a None value. Here, we tell Rust that we
    // mean for absent_number to be of type Option<i32>.

    // You cannot do this
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;
    // cannot add `Option<i8>` to `i8`

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let n = y.unwrap();
    let sum = x + n;
    println!("Sum is {}", sum);

    // Note:
    // Unwrapping a `Some` variant will extract the value wrapped.
    // Unwrapping a `None` variant will `panic!`

    // Matching with Option<T>
    let five = Some(5);
    let six = add_one(five);
    let none = add_one(None);
    println!("Six: {}", six.unwrap());
    // ERRRO: pritnln!("None: {}", none.unwrap());
    // Cannot unwrap None
}

fn add_one(n: Option<i32>) -> Option<i32> {
    match n {
        None => None,
        Some(i) => Some(i+1),
    }
}
