use std::io;
use problem2::sum_one_to_n;

fn main() {

    loop {
        println!("Give me a number, any number!");

        // this variable can be used to store the string input from the user
        let mut number = String::new();

        io::stdin().read_line(&mut number).expect("Huh?");
        match number.trim().parse() {
            Ok(num) => {
                let result = sum_one_to_n(num);
                println!("The sum of 1 to {} is: {}", num, result);
            },
            Err(_) => continue,
        }
    }
}
