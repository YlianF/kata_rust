use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let mut lines = input.lines();
    lines.next();

    let second_line = lines
        .next()
        .expect("Il n'y a pas de deuxième ligne")
        .trim();

    let count: i32 = second_line
        .split_whitespace()
        .filter_map(|word| word.parse::<i32>().ok())
        .filter(|&num| num < 0)
        .map(|x|x.abs())
        .sum();

    println!("{}", count);
}