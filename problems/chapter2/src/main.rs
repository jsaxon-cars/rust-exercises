use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
    
    println!("Please input your guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    loop {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_elements() {
        let empty_vec = Vec::new();

        let result = main();

        assert_eq!(result, None);
    }

    #[test]
    fn test_some_elements() {
        let myvec = vec![1, 5, 3, 8, 0];

        let result = main();

        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_negative_elements() {
        let myvec = vec![-1, -5, -3, -8];

        let result = main();

        assert_eq!(result, Some(-1));
    }
}
