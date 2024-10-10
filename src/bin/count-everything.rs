use std::io;

fn main() {
    let mut input: String = String::new();

    let mut nl = 0;

    while let Ok(bytes) = io::stdin().read_line(&mut input) {
        if bytes == 0 {
            break;
        }
        nl += 1;
    }

    // This was an approach when I got a list of the words, but count() already count everything so not needed.
    // let words: Vec<&str> = input.split_whitespace().collect();
    // println!("{:?}",words);
    
    // Pretty useful operators to work with strings and arrays keep it as reference.
    let nw = input.split_whitespace().count();
    let nc = input.chars().count();

    println!("Lines: {}, Words: {}, Characters: {}", nl, nw, nc);
}