mod shapes;
use shapes::{Area, Circle, Rect};
// We need to import the Area trait if we want to use it from a Struc that implements the trait.
// This means the implementation of the Area trait for f64 is safe and not applied to all the code base.

fn main() {
    let rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
    };

    let circ = Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };

    println!("{}", circ.area());
    println!("{}", rect.area());
}
