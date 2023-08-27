mod shapes;

use crate::shapes::{area::Area, circle::Circle, rect::Rect};

fn main() {
    let rect = Rect::default();

    for point in &rect {}

    println!("{}", rect)
}
