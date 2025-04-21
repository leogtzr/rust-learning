fn main() {
    // println!("Hello World!");
    let sum1: i32 = sum_plus_one(23);
    println!("Value: {}", sum1);
}

fn sum_plus_one(x: i32) -> i32 {
    // return x + 1; should work too
    x + 1
}

fn another_function() {
    println!("Another function");
}
