use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let lines = input.lines();

    for (index, line) in lines.enumerate() {
        if index == 0 {
            let words: Vec<&str> = line.split_whitespace().collect();
            if words.len() >= 2 {
                if let Ok(second_number) = words[1].parse::<i32>() {
                    println!("{}", second_number);
                    break;
                }
            }
        }
    }
}