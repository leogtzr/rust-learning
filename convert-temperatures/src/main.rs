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
        print!("\nTemperature: ");
        io::stdout().flush().unwrap();

        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        // parse() 
        let temperature: u32 = match temperature.trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("{}°C = {:.1}°F", temperature, to_fahrenheit(temperature));
        println!("{}°F = {:.1}°C", temperature, to_celsius(temperature));    
    }
}
