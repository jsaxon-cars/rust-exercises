use chapter4::prefix;
use chapter4::suffix;
use std::io;

fn main() {
    let s = "Middle";
    let s = prefix(s, "Front");
    println!("Word is now: {}", s);
    let s = suffix(s, "Back");
    println!("Word is now: {}", s);
}
