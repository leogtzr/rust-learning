fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let some_number: Option<i32> = Some(5);
    let some_char: Option<char> = Some('e');

    match some_number {
        Some(x) => {
            println!("The num is: {}", x);
        },
        None => {}
    };

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("None is: {:?}", none);

    match six {
        None => {
            println!("No value");
        },
        Some(val) => {
            println!("Result is: {}", val);
        }
    }
}
