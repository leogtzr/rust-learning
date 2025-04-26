/* Previous code:
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
*/
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    let my_name = String::from("Leonardo ");
    change(&my_name);
}

fn change(some_string: &String) {
    some_string.push_str(" Gutiérrez");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}