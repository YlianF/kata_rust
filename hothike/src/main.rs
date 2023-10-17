use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let mut lines = input.lines();
    lines.next();

    
    let mut nbl = 1;
    let mut best = 100;
    let mut bestl = 100;
    let mut output = 0;

    for line in lines {
        let jours: Vec<i32> = line
            .split_whitespace()
            .map(|word| word.parse().expect("ALLO"))
            .collect();

        for i in 0..jours.len()-2 {
            if jours[i] < best && jours[i+2] < best {
                if jours[i] < jours[i+2] {
                    output = jours[i+2];
                } else {
                    output = jours[i];
                }
                    
                best = output;
                bestl = i+1;
            }
        }
    }

    println!("{} {}", bestl, output);
}