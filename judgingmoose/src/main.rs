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

        if first < 0 || first > 20 || second < 0 || second > 20 {
            return;
        }

        if first == second && first != 0 {
            println!("Even {}", second*2);
        } else if first < second {
            println!("Odd {}", second*2);
        } else if first > second {
            println!("Odd {}", first*2);
        } else {
            println!("Not a moose")
        }
    }
}