use std::io;

fn main() {
    println!("Rate die Zahl!");

    println!("Bitte gib Deine Schätzung ein.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Fehler beim Lesen der Deiner Eingab!");
    
    println!("Du hast geschätzt: {guess}");
}
