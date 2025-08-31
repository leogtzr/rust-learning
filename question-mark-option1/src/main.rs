fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    match last_char_of_first_line("Leo") {
        Some(last_char) => println!("Is: {}", last_char),
        None => {
            println!("Nada, alabeibe")
        }
    }
}
