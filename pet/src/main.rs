use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let lines = input.lines();

    let mut nbl = 0;
    let mut best = 0;
    let mut bestl = 0;
    for line in lines {
        nbl += 1 ;
        let mut compt = 0;

        let nb: Vec<i32> = line
            .split_whitespace()
            .map(|word| word.parse().expect("ALLO"))
            .collect();

        compt += nb[0] + nb[1] + nb[2] + nb[3];

        if compt > best {
            best = compt;
            bestl = nbl;
        }
    }
    println!("{} {}", bestl, best);
}