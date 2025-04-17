use std::io;

fn main() {
    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Result<x, y> is an enumeration.
    // Variant: each possible state is a variant.
    // :: associated function.
    // {} is a placeholder. 

    println!("You guesses: {}", guess);

    let x = 5;
    let y = 10;
    println!("x = {x} and y + 2 = {}", y + 2);
}
