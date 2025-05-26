use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    // This won't be executed since it already exists.
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
}
