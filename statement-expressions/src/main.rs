fn five() -> i32 {
    5
}

fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("Value: {}", y);

    let z = {
        let x = 3;
        x + 1
    };

    let x = five();
    println!("The value of x is: {x}");

    let p_one = plus_one(x);
    println!("x + 1 is: {p_one}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}