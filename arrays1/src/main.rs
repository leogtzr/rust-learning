fn main() {
    let a = [1, 2, 3, 4, 5];
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    println!("First element: {}", months[0]);

    let a2: [i32; 5] = [1, 2, 3, 4, 5];
    for month in months {
        println!("Mes: {}", month);
    }

    for month in months.iter() {
        match month {
            &"January" => {
                println!("Found the month alv");
            },
            _ => {}
        }
    }

    let first_month: &str = months[0];
    println!("Primer mes: {}", first_month);

    for month in months {
        println!("Mes: {}", month);
    }
}
