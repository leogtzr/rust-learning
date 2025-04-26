use std::io;
use std::io::Write;

fn to_celsius(temperature: u32) -> f32 {
    (temperature as f32) * (9.0 / 5.0) + 32.0
}

fn to_fahrenheit(temperature: u32) -> f32 {
    ((temperature as f32) - 32.0) * (5.0 / 9.0)
}

fn main() {
    loop {
        print!("\nnth: ");
        io::stdout().flush().unwrap();

        let mut nth = String::new();
        io::stdin()
            .read_line(&mut nth)
            .expect("Failed to read line");

        let nth: u32 = match nth.trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        let mut a: u32 = 0;
        let mut b: u32 = 1;
        let mut tmp: u32 = 0;
        let mut count = 1;

        while count < nth {
            tmp = a + b;
            a = b;
            b = tmp;

            count += 1;
        }

        println!("= {a}"); 
    }
}
