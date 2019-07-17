use simple_error::SimpleError;
use std::str::FromStr;
use std::io;

use problem4::make_flower_box;

pub enum MaybeWord {
    Word(String),
    Blank
}

impl FromStr for MaybeWord {
    type Err = SimpleError;

    /// Takes a string slice as input and either parses it into a `GameElement`
    /// or returns an error.
    fn from_str(word: &str) -> Result<Self, Self::Err> {

        if word.len() > 0 {
            Ok(MaybeWord::Word(String::from(word)))
        } else if word.len() == 0 {
            Ok(MaybeWord::Blank)
        } else {
            Err(SimpleError::new("Impossible!"))
        }
    }
}


// Console-based rock/paper/scissors game. Make sure to fix the broken
// code in `game_element.rs`.
fn main() {
    println!("Let's Make Flower Boxes!");

    let mut flowers: Vec<String> = vec![];
    let mut raw_input = String::new();

    loop {
        println!("Input a flower.  Hit Return to build Flower Box: ");

        io::stdin().read_line(&mut raw_input).expect("Failed to read line");

        match raw_input.trim().parse() {
            Err(_) => {
                println!("Not possible!");
                continue;
            },

            Ok(maybe_flower) => {
                match maybe_flower {
                    MaybeWord::Word(flower) => {
                        println!("Added flower: {}\n", &flower);
                        flowers.push(flower);
                        raw_input = String::new();
                    },
                    MaybeWord::Blank => {
                        let mut new_flowers: Vec<&str> = vec![];
                        for flower in &flowers {
                            new_flowers.push(&flower)
                        }
                        println!("
We got all the flowers!

{:?}

Let's plant them!

{}
                        ", &flowers, make_flower_box(new_flowers));

                    }
                }

            }
            
        };

    }

}

#[cfg(test)]
mod test {
    // use super::*;

    // #[test]
    // fn test_maybe_word() {
    //     let mw: MaybeWord = "THIS".from_str();
    //     assert_eq!(mw, MaybeWord::Word("THIS"));
    // }

}