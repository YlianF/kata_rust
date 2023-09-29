use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let mut lines = input.lines();
    lines.next();
    
    for line in lines {
        let n= line.trim().to_string();
        println!("{}", n.len());
    }
}