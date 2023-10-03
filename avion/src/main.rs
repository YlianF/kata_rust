use std::io;

fn main() {
    let mut cia_blimps: Vec<usize> = Vec::new();

    for (index, line) in io::stdin().lines().enumerate() {
        let registration_code = line.expect("Erreur lors de la lecture de l'entrée");

        // Vérifier si le code de registation contient "FBI"
        if registration_code.contains("FBI") {
            cia_blimps.push(index + 1); // L'index commence à 0, donc on ajoute 1 pour obtenir le numéro de ligne
        }
    }

    if cia_blimps.is_empty() {
        println!("HE GOT AWAY!");
    } else {
        for (i, blimp) in cia_blimps.iter().enumerate() {
            print!("{}", blimp);
            if i < cia_blimps.len() - 1 {
                print!(" ");
            }
        }
        println!();
    }
}