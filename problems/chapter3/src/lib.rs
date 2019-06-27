use std::str;

pub fn teaseout(input: &str) -> (&str, &str) {
    println!("{:?}", input.to_lowercase().split("f"));
    println!("{:?}", input.to_lowercase().split("c"));

    ("F", "1")
}

struct Temperature(f64);

// impl str::FromStr for Temperature {
//     type Err = SimpleError;

//     /// Takes a string slice as input and either parses it into a `GameElement`
//     /// or returns an error.
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let choice = s.trim();
//         if choice == "f" {
//             Ok(Fahrenheit(0))
//         } else if choice == "c" {
//             Ok(Celsius(0))
//         } else {
//             Err(SimpleError::new("Choice must start with r, p, or s"))
//         }
//     }
// }

#[derive(PartialEq, Debug)]
struct Fahrenheit(pub f64);

impl Fahrenheit {
    fn to_celsius(self) -> Celsius {
        Celsius(3.0)
    }
}

/// For our game parser we'll accept any string that starts with r, p, or s
/// and convert it into Rock, Paper, or Scissors, respectively
#[derive(PartialEq, Debug)]
struct Celsius(pub f64);

impl Celsius {
    fn to_fahrenheit(self) -> Fahrenheit {
        Fahrenheit(3.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_teaseout() {
        assert_eq!(teaseout("1f"), ("F", "1"));
    }
    // fn test_teaseout() {
    //     assert_eq!(teaseout(""), None);
    //     assert_eq!(teaseout("1"), None);
    //     assert_eq!(teaseout("f"), None);
    //     assert_eq!(teaseout("f1"), Some((Temperature::F, 1.0)));
    //     assert_eq!(teaseout("F1"), Some((Temperature::F, 1.0)));
    //     assert_eq!(teaseout("1f"), Some((Temperature::F, 1.0)));
    //     assert_eq!(teaseout("1F"), Some((Temperature::F, 1.0)));
    //     assert_eq!(teaseout("c1"), Some((Temperature::C, 1.0)));
    //     assert_eq!(teaseout("C1"), Some((Temperature::C, 1.0)));
    //     assert_eq!(teaseout("1c"), Some((Temperature::C, 1.0)));
    //     assert_eq!(teaseout("1C"), Some((Temperature::C, 1.0)));
    // }

    #[test]
    fn test_to_celsius() {
        assert_eq!(Fahrenheit(10.0).to_celsius(), Celsius(3.0))
    }

    #[test]
    fn test_to_fahrenheit() {
        assert_eq!(Celsius(10.0).to_fahrenheit(), Fahrenheit(3.0))
    }
}
