use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let lines = input.lines();

    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        let first= words[0].parse::<i32>().expect("Entier");
        let second= words[1].parse::<i32>().expect("Entier");

        if first < -1000 || first > 1000 || second < -1000 || second > 1000 {
            return;
        }
        
        println!("{}", second + (second-first))
    }
}