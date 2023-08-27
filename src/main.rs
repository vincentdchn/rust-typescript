mod shapes;

use crate::shapes::{circle::Circle, collisions::Collidable, rect::Rect};

fn main() {
    let rect = Rect::default();
    let rect2 = Rect::default();

    let circle = Circle {
        x: 0.0,
        y: 0.0,
        radius: 1.0,
    };

    let circle2 = Circle {
        x: 1.5,
        y: 1.5,
        radius: 4.0,
    };

    rect.collide(&rect2);
    circle.collide(&circle2);
}
