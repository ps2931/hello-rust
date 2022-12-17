// Traits: Defining Shared Behaviour

pub fn run() {
    // A trait defines functionality a particular type has and can share with other types.
    // Lets define a trait for finding quadratic root of a number.
    println!("{} {}", quadratic_root(100f64), quadratic_root(100f32));
}

// The meaning of the "HasSquareRoot" trait is that the "sq_root" function can be invoked on
// every type having the "HasSquareRoot" capability, or, as it is usually said, every type that
// satisfies the "HasSquareRoot" trait.
trait HasSquareRoot {
    fn sq_root(self) -> Self;
}

impl HasSquareRoot for f32 {
    fn sq_root(self) -> Self {
        f32::sqrt(self)
    }
}

impl HasSquareRoot for f64 {
    fn sq_root(self) -> Self {
        f64::sqrt(self)
    }
}

fn quadratic_root<T>(x: T) -> T
where
    T: HasSquareRoot, // Trait bound
{
    x.sq_root().sq_root()
}
