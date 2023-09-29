use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let lines = input.lines();


    for line in lines {
        let n = line.parse::<i32>().expect("Entier");
        if n < 1 || n > 100 {
            return;
        }
        for i in 0..n {
            println!("{} Abracadabra", i+1);
        }
    }
}
