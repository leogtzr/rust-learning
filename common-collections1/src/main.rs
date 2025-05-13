fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = vec![1, 2, 3, 4];
    v2.push(5);

    println!("Value is: {:?}", v2);

    let edades = vec![33, 42, 12, 78, 2, 57];
    let third_age = edades[3];
    let third: &i32 = &edades[3];

    println!("Third age is: {}", third_age);
    // Parece que un derreferencing se hace automáticamente.
    let sum = *third + 1;

    println!("The value is: {}", sum);

    let last_edad: Option<&i32> = edades.get(5);
    match last_edad {
        Some(last) => println!("The last element is: {last}"),
        None => println!("There is no element...")
    }
}
