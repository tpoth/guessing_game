use std::io;
use rand::Rng;

fn main() {
    println!("Rate die Zahl!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Die Geheimzahl ist: {secret_number}");

    println!("Bitte gib Deine Schätzung ein.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Fehler beim Lesen der Deiner Eingab!");
    
    println!("Du hast geschätzt: {guess}");
}
