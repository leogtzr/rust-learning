use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Scores: {:?}", scores);

    let team_name = String::from("Blue");
    // let score = scores.get(&team_name).unwrap();
    // println!("Value: {}", score);
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // .copied() -> regresa un Option<X>, en lugar de un Option<&X>
    // Básicamente lo hemos hecho para ahorrarnos el tener que hacer un deref.

    println!("Score: {}", score);
}
