use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let lines = input.lines();

    for line in lines {
        let n = line.parse::<f64>().expect("Entier");
        if n < 1.0 || n>100000.0 {
            return;
        }
        let string = format!("{:.2}", n/4.0);
        println!("{}", string)
    }
}