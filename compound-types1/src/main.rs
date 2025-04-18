fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup1: {}", tup.0);
    println!("tup2: {}", tup.1);
    println!("tup3: {}", tup.2);

    let (x, y, z) = tup;
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
}
