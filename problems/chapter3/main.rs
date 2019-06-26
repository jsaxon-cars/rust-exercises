use std::io;
use chapter3::;

fn main() {

    loop {
        println!("Give me a number, any POSITIVE INTEGER!");

        // This variable can be used to store the string input from the user
        // Strings are weird in Rust.
        // let mut number = String::new();

        // // HOW DO YOU MAKE IT BREAK.
        // io::stdin().read_line(mut number).expect("Huh?");
        let number = my_read_line();
        match number.trim().parse() {
            Ok(num) => {
                println!("The sum of 1 to {} is: {}", num, sum_one_to_n(num));
            },
            Err(_) => continue,
        }
    }
}

fn my_read_line() -> String {
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(count) => println!("Took {} characters.", count - 1),
        Err(_) => println!("Never gonna happen."),
    }
    line.trim().to_string()
}
