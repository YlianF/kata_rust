use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let mut lines = input.lines();
    lines.next();

    let mut compt = 0.0;
    for line in lines {
        let nb: Vec<f64> = line
            .split_whitespace()
            .map(|word| word.parse().expect("ALLO"))
            .collect();
        compt += nb[0] * nb[1];
    }
    println!("{:.3}", compt);
}