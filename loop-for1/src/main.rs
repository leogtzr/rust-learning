fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    println!("-------------------------------------------------------");

    for element in a {
        println!("The value is: {element}");
    }
}
