use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let lines = input.lines();

    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        let first = words[0].parse::<i64>().expect("Entier");
        let second = words[1].parse::<i64>().expect("Entier");
        let third = words[2].parse::<i64>().expect("Entier");
        for i in 1..third+1 {
            if i % first == 0 && i % second == 0 {
                println!("FizzBuzz");
            } else if i % second == 0 {
                println!("Buzz");
            } else if i % first == 0 {
                println!("Fizz");
            } else {
                println!("{}", i);
            }
        }
    }
}