use std::vec::Vec;
use std::str;

/// Takes a vector of string slices and formats them
/// line by line in a flower box, e.g.
/// vec!["one", "two", "three"] becomes:
///
/// *********
/// * one   *
/// * two   *
/// * three *
/// *********
///
pub fn make_flower_box(elems: Vec<&str>) -> String {

    let mut flower_box = String::new();
    // get max length of vector add four to it.
    let max = get_max_line_length(&elems);
    let border = format_border("*", max + 4);
    flower_box.push_str(&border);
    for word in elems.iter() {
        flower_box.push_str(&format_line(word, max));
    }; 
    flower_box.push_str(&border);

    flower_box

}

/// Creates a string with a border and space padding.
///
/// Examples
/// ```
/// use problem4::format_line;
/// let actual = format_line("hello", 10);
/// let expected = String::from("* hello      *\n");
/// assert_eq!(actual, expected)
/// ```
pub fn format_line(s: &str, max_length: usize) -> String {
    match max_length >= s.len() {
        true => format!("* {}{} *\n", s, " ".repeat(max_length - &s.len())),
        _ => format!("* {} *\n", &s[..max_length]),
    }
    
    
}

/// Creates a border that can be used at the top and bottom
/// of our flower box.
///
/// Examples
/// ```
/// use problem4::format_border;
/// let actual = format_border("*", 5);
/// let expected = String::from("*****\n");
/// assert_eq!(actual, expected)
/// ```
pub fn format_border(s: &str, length: usize) -> String {
    match s.len() {
        0 => String::from("\n"),
        _ => s[0..1].repeat(length) + "\n",
    }
}

/// Gets the longest line length from `elems`.
///
/// Examples
/// ```
/// use problem4::get_max_line_length;
/// let elems = vec!["fizz", "buzz", "fizzbuzz"];
/// assert_eq!(get_max_line_length(&elems), 8);
/// ```
pub fn get_max_line_length(elems: &Vec<&str>) -> usize {
    match &elems[..] {
        [] => 0,
        [first] => first.len(),
        _ => 
        elems[0].len()
        .max(get_max_line_length(&elems[1..].to_vec())),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_format_line() {
        assert_eq!("*  *\n", format_line(&"", 0));
        assert_eq!("*  *\n", format_line(&"hello", 0));
        assert_eq!("*   *\n", format_line(&"", 1));
        assert_eq!("*   *\n", format_line(&" ", 1));
        assert_eq!("*    *\n", format_line(&" ", 2));
        assert_eq!("* he *\n", format_line(&"hello", 2));
        assert_eq!("* hello *\n", format_line(&"hello", 5));
        assert_eq!("* hello  *\n", format_line(&"hello", 6));
        assert_eq!("* hello   *\n", format_line(&"hello", 7));
    }

    #[test]
    fn test_format_border() {
        assert_eq!("\n", format_border(&"", 0));
        assert_eq!("\n", format_border(&"*", 0));
        assert_eq!("*\n", format_border(&"*", 1));
        assert_eq!("*\n", format_border(&"**", 1));
        assert_eq!("**\n", format_border(&"*", 2));
        assert_eq!("xxxx\n", format_border(&"x", 4));
    }

    #[test]
    fn test_max_line_length() {
        assert_eq!(0, get_max_line_length(&vec![]));
        assert_eq!(0, get_max_line_length(&vec![""]));
        assert_eq!(0, get_max_line_length(&vec!["",""]));
        assert_eq!(3, get_max_line_length(&vec!["foo"]));
        assert_eq!(4, get_max_line_length(&vec!["foo", "foob"]));
        assert_eq!(6, get_max_line_length(&vec!["foobey", "foob"]));
        assert_eq!(4, get_max_line_length(&vec!["", "foob"]));
        assert_eq!(5, get_max_line_length(&vec!["", "foob", "mooey"]));
    }

    #[test]
    fn test_empty_box() {
        assert_eq!(String::from("****\n****\n"), make_flower_box(vec![]));
    }

    #[test]
    fn test_box() {
        let expected = String::from("\
        *********\n\
        * one   *\n\
        * two   *\n\
        * three *\n\
        * four  *\n\
        * five  *\n\
        * six   *\n\
        * seven *\n\
        *********\n");

        let test_vec = vec!["one", "two", "three", "four", "five", "six", "seven"];

        assert_eq!(expected, make_flower_box(test_vec));
    }

}

// ['yes', 'no', 'maybe so']
// 
// ************
// * yes      *
// * no       *
// * maybe so *
// ************
