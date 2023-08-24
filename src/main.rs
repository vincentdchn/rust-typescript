enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false;
    }

    fn is_green_parts(&self) -> bool {
        match self {
            Color::Red => false,
            Color::Green => false,
            Color::Blue => true,
            Color::Yellow => true,
        }
    }
}

fn print_color(color: Color) {
    match color {
        Color::Blue => println!("Blue"),
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Yellow => println!("Yellow"),
    }
}

fn main() {
    let foo = Color::Green;

    foo.is_green()
}
