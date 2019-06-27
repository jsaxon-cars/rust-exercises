use chapter3::teaseout;
use std::io;

fn main() {
    loop {
        let (a, b) = teaseout("fred");

        println!("Input a temperature with an F or C suffix:\n");

        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(temperature) => {
                println!("\nTook {} characters.", temperature);
                temperature
            }
            Err(_) => {
                println!("Nope");
                continue;
            }
        };
        println!("Convering {} to ?", temperature);
    }
}
