use rand::distributions::Distribution;
use rand::distributions::Standard;
use std::cmp::Ordering;
use simple_error::SimpleError;
use std::str::FromStr;
use std::fmt;

/// The base type for representing Rock, Paper, and Scissors, which are all the
/// possible choices in our game.
#[derive(PartialEq, Eq, Debug)]
pub enum GameElement {
    Rock,
    Paper,
    Scissors,
}

/// Follows the standard ordering rules for rock paper scissors, where
/// rock < paper < scissors < rock.
///
/// # Examples
///
/// ```
/// use problem3::game_element::GameElement::*;
/// let comparison = Rock < Paper;
/// assert_eq!(comparison, true);
///```
impl Ord for GameElement {

    fn cmp(&self, other: &Self) -> Ordering {

        use GameElement::*;
        use Ordering::*;

        // This one is tricky. What are all the cases we need to cover? Do we have
        // tests for all the cases? The broken function below returns Less
        // for Rock compared to Paper (meaning `Rock < Paper == true`), but we
        // need to cover all cases.
        match (self, other) {
            (Paper, Rock) | (Rock, Scissors) | (Scissors, Paper) => Greater,
            (Rock, Paper) | (Scissors, Rock) | (Paper, Scissors) => Less,
            _ => Equal
        }
    }
}

/// Since `GameElement` should allow for all of its variants to be
/// compared, we can define the partial ordering via the `cmp` method from
/// `Ord`.
impl PartialOrd for GameElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Allows callers to randomly generate game choices.
impl Distribution<GameElement> for Standard {

    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> GameElement {

        // randomly chooses 1, 2, or 3
        let n: u32 = rng.gen_range(1, 4);

        match n {
            1 => GameElement::Rock,
            2 => GameElement::Paper,
            _ => GameElement::Scissors,
        }

    }
}

/// Console-friendly string representation of each element.
impl fmt::Display for GameElement {

    /// FIX ME!
    /// This displays a user friendly string representation of all three
    /// `GameElement` variants.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// For our game parser we'll accept any string that starts with r, p, or s
/// and convert it into Rock, Paper, or Scissors, respectively
impl FromStr for GameElement {
    type Err = SimpleError;

    /// Takes a string slice as input and either parses it into a `GameElement`
    /// or returns an error.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let choice = s.trim();
        if  choice == "r" {
            Ok(GameElement::Rock)
        } else if choice == "s" {
            Ok(GameElement::Scissors)
        } else if choice == "p" {
            Ok(GameElement::Paper)
        } else {
            Err(SimpleError::new("Choice must start with r, p, or s"))
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ordering() {
        use GameElement::*;

        // does this test everything we need?
        assert!(Rock < Paper && Paper < Scissors && Scissors < Rock);
        assert!(Paper > Rock && Scissors > Paper && Rock > Scissors);
        assert!(Paper == Paper && Scissors == Scissors && Rock == Rock);
    }


    // add additional tests to make sure we can parse game elements from
    // strings and also display them

}
