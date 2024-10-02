use std::io;

fn main() {
    let mut input = String::new();

    // Read a line, need to create a reference to put the value there
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");

    println!("{}",&input);
}