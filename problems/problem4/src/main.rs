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
    println!("Let's make flower boxes!");

    let mut word_list: Vec<&str> = vec![];

    loop {
        println!("Input a word: ");

        let mut word = String::new();

        io::stdin().read_line(&mut word).expect("Failed to read line");

        match word.trim().parse() {
            Ok(MaybeWord::Blank) =>  {
                make_flower_box(word_list);
            },
            Ok(MaybeWord::Word(new_word)) => {
                // Add word to word_list
                println!("WORD IS: {}", &new_word);
                word_list.push(&new_word);
            },
            Err(_) => {
                println!("Not possible!");
                continue;
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