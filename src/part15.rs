// Closures in Rust
// Reference: https://www.youtube.com/watch?v=bgZa9VRBhYU&t=593

pub fn run() {
    let x = 21;
    let get_answer = |y:i32| x + y;
    println!("{:?}", get_answer(21));

    // The type of get_answer is `|i32| -> i32`.

    // function not a closure
    fn add(x: i32, y:i32) -> i32 { x + y }
    let f = add;

    let result = f(1, 2);
    println!("{}", result); // 3

    // closure
    let f1 = |x:i32, y:i32| { x + y };
    let result = f1(1, 2);
    println!("{}", result); // 3

    // closure - more consise syntax
    let f2 = |x,y| x+y;
    let result = f2(1, 2);
    println!("{}", result); // 3


    fn calc_and_print(x: i32, y:i32, calculator: fn(i32, i32) -> i32) {
        let result = calculator(x, y);
        println!("{}", result);
    }

    calc_and_print(1, 3, add); // 4
    calc_and_print(3, 2, |x, y| x * y); // 6


    fn calc_and_print_v2(x: i32, y:i32, calculator: Box<dyn Fn(i32, i32) -> i32 + '_>) {
        let result = calculator(x, y);
        println!("{}", result);
    }

    calc_and_print_v2(5, 6, Box::new(add)); // 11
    calc_and_print_v2(5, 6, Box::new(|x, y| x+y)); // 11

    // Now we can also pass a closure with capturing
    // to calc_and_print_v2
    let z = 3;
    calc_and_print_v2(1, 2, Box::new(|x,y| x+y+z));
}
