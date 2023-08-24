enum Color {
    Red,
    Green,
    Blue,
}

fn print_color(color: Color) {
    match color {
        Color::Blue => println!("Blue"),
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
    }
}

fn main() {
    print_color(Color::Blue)
}
