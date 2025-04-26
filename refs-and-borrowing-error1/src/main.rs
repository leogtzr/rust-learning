fn main() {
    let mut s = String::from("Leo");

    change(&mut s);

    println!("The value is: {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(" Gutiérrez");
}
