////////////////////////////////////////
// Structs to Structure Related Data
////////////////////////////////////////

// A structure is a custom data type.
struct SeaCreature { 
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

// Tuple struct
struct Location(i32, i32);

// Sturcts do not have to have any fields at all.
struct Marker;

pub fn run() {
    // When we instantiate a struct in our code our program creates the associated field data side by side in memory.
    let ferris = SeaCreature {
        animal_type: String::from("Crab"),
        name: String::from("Ferris"),
        arms: 2, 
        legs: 4,
        weapon: String::from("claw"),            
    };
    
    println!(
        "{} is a {}. They have {} arms, {} legs, and a {} weapon",
        ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
    );
 
    // We used the owned String type rather than the &str string slice type. This is a deliberate choice because
    // we want each instance of this struct to own all of its data and for that data to be valid for as long as 
    // the entire struct is valid.
    
    // If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field.
    // Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.
    
    // Tuple struct
    let loc = Location(42, 32);
    println!("{}, {}", loc.0, loc.1);

    // Unit struct
    let m = Marker;
            
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("Rectangle is : {:?}", rect1);
    println!("Area of rectangle is {}", area(&rect1));
    
    // Method syntax
    let rect2 = Rectangle {
        width: 25,
        height: 30,
    };
    println!("Area of rectangle is {}", rect2.area());
    
    // Methods with more parameters
    let rect1 = Rectangle { width: 30, height: 50, };
    let rect2 = Rectangle { width: 10, height: 40, };
    let rect3 = Rectangle { width: 60, height: 45, };
    
    // write a method can_hold() to compare two rectangles
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    // Associated function calling
    let square = Rectangle::square(3);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// Everything within this impl block will be associated with the Rectangle type.
// All functions defined within an impl block are called associated functions because
// they’re associated with the type named after the impl. We can also define associated 
// functions that don’t have self as their first parameter (and thus are not methods) 
// because they don’t need an instance of the type to work with.
impl Rectangle {
    // The &self is actually short for `self: &Self`. Within an impl block, the type Self is an alias
    // for the type that the impl block is for. Methods must have a parameter named self of type Self
    // for their first parameter, so Rust lets you abbreviate this with only the name self in the first
    // parameter spot. Note that we still need to use the & in front of the self shorthand to indicate 
    // this method borrows the Self instance.
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Apart from self, can_hold takes an immutable borrow of another Rectangle.
    // The return value of can_hold will be a Boolean, and the implementation will
    // check whether the width and height of self are both greater than the width 
    // and height of the other Rectangle.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}