use chapter4::my_prefix;
use chapter4::my_suffix;

// use std::io;

fn main() {
    let s = "Middle";
    let s = my_prefix(s.to_string(), "Front".to_string());
    println!("Word is now: {}", s);
    let s = my_suffix(s, "Back".to_string());
    println!("Word is now: {}", s);
}
