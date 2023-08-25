fn main() {
    let file_name = std::env::args()
        .nth(1)
        .expect("The file name to be passed in");

    let file = std::fs::read_to_string(file_name).expect("Unable to read file");

    file.lines().for_each(|line| {
        if let Ok(value) = line.parse::<usize>() {
            println!("{}", value)
        } else {
            println!("Line not a number")
        }
    })
}
