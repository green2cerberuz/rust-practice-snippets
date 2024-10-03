use std::io;

fn main() {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading the input line");
    
    println!("{}", input.len() -1) // remove the \n at the end of the input string
}