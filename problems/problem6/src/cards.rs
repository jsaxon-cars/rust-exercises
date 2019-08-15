use std::fmt;
use rand::seq::SliceRandom;

pub struct Deck {
    cards: Vec<Card>
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Spades => write!(f, "{}", "♠"),
            Suit::Hearts => write!(f, "{}", "♥"),
            Suit::Diamonds => write!(f, "{}", "♦"),
            Suit::Clubs => write!(f, "{}", "♣"),
        }
    }
}

impl Rank {
    pub fn value(&self) -> u8 {
        use Rank::*;
        match self {
            Two     => 2,
            Three   => 3,
            Four    => 4,
            Five    => 5,
            Six     => 6,
            Seven   => 7,
            Eight   => 8,
            Nine    => 9,
            Ten     => 10,
            Jack    => 10,
            Queen   => 10,
            King    => 10,
            Ace     => 11,
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Rank::*;
        let rank: String = match self {
            Jack => String::from("J"),
            Queen => String::from("Q"),
            King => String::from("K"),
            Ace => String::from("A"),
            _ => self.value().to_string(),
        };
        write!(f, "{}", rank)
    }
}

impl Card {
    pub fn default() -> Card {
        Card {
            rank: Rank::Ace,
            suit: Suit::Spades
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}{}]", self.rank, self.suit)
    }
}

impl Deck {

    pub fn new() -> Self {
        let mut deck = Self {
            cards: Vec::new()
        };
        deck.load();
        deck
    }

    pub fn reset(&mut self) {
        self.cards.clear();
        self.load();
    }

    pub fn deal_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    fn load(&mut self) {
        let suits = vec!(Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs);
        let ranks = vec!(
            Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, 
            Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, 
            Rank::Ace, );
        for suit in &suits {
            for rank in &ranks {
                let card = Card {rank: rank.clone(), suit: suit.clone()};
                self.cards.push(card);
            }
        }

    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_suit_display() {
        assert_eq!(format!("{}", Suit::Spades), "♠");
        assert_eq!(format!("{}", Suit::Hearts), "♥");
        assert_eq!(format!("{}", Suit::Diamonds), "♦");
        assert_eq!(format!("{}", Suit::Clubs), "♣");
    }

    #[test]
    fn test_rank_display() {
        assert_eq!(format!("{}", Rank::Two), "2");
        assert_eq!(format!("{}", Rank::Three), "3");
        assert_eq!(format!("{}", Rank::Four), "4");
        assert_eq!(format!("{}", Rank::Five), "5");
        assert_eq!(format!("{}", Rank::Six), "6");
        assert_eq!(format!("{}", Rank::Seven), "7");
        assert_eq!(format!("{}", Rank::Eight), "8");
        assert_eq!(format!("{}", Rank::Nine), "9");
        assert_eq!(format!("{}", Rank::Ten), "10");
        assert_eq!(format!("{}", Rank::Jack), "J");
        assert_eq!(format!("{}", Rank::Queen), "Q");
        assert_eq!(format!("{}", Rank::King), "K");
        assert_eq!(format!("{}", Rank::Ace), "A");
    }

    #[test]
    fn test_load_deck() {
        let mut deck = Deck {
            cards: Vec::new()
        };
        deck.load();
        assert_cards_exist(&deck)
    }

    #[test]
    fn test_shuffle_deck() {
        let mut deck = Deck {
            cards: Vec::new()
        };
        deck.load();
        deck.shuffle();
        assert_cards_exist(&deck)
    }

    fn assert_cards_exist(deck: &Deck) {
        use Rank::*;
        use Suit::*;
        assert_eq!(deck.cards.len(), 52);
        assert!(exists(&deck, Ace, Spades));
        assert!(exists(&deck, Two, Spades));
        assert!(exists(&deck, Three, Spades));
        assert!(exists(&deck, Four, Spades));
        assert!(exists(&deck, Five, Spades));
        assert!(exists(&deck, Six, Spades));
        assert!(exists(&deck, Seven, Spades));
        assert!(exists(&deck, Eight, Spades));
        assert!(exists(&deck, Nine, Spades));
        assert!(exists(&deck, Ten, Spades));
        assert!(exists(&deck, Jack, Spades));
        assert!(exists(&deck, Queen, Spades));
        assert!(exists(&deck, King, Spades));
        assert!(exists(&deck, Ace, Hearts));
        assert!(exists(&deck, Two, Hearts));
        assert!(exists(&deck, Three, Hearts));
        assert!(exists(&deck, Four, Hearts));
        assert!(exists(&deck, Five, Hearts));
        assert!(exists(&deck, Six, Hearts));
        assert!(exists(&deck, Seven, Hearts));
        assert!(exists(&deck, Eight, Hearts));
        assert!(exists(&deck, Nine, Hearts));
        assert!(exists(&deck, Ten, Hearts));
        assert!(exists(&deck, Jack, Hearts));
        assert!(exists(&deck, Queen, Hearts));
        assert!(exists(&deck, King, Hearts));
        assert!(exists(&deck, Ace, Diamonds));
        assert!(exists(&deck, Two, Diamonds));
        assert!(exists(&deck, Three, Diamonds));
        assert!(exists(&deck, Four, Diamonds));
        assert!(exists(&deck, Five, Diamonds));
        assert!(exists(&deck, Six, Diamonds));
        assert!(exists(&deck, Seven, Diamonds));
        assert!(exists(&deck, Eight, Diamonds));
        assert!(exists(&deck, Nine, Diamonds));
        assert!(exists(&deck, Ten, Diamonds));
        assert!(exists(&deck, Jack, Diamonds));
        assert!(exists(&deck, Queen, Diamonds));
        assert!(exists(&deck, King, Diamonds));
        assert!(exists(&deck, Ace, Clubs));
        assert!(exists(&deck, Two, Clubs));
        assert!(exists(&deck, Three, Clubs));
        assert!(exists(&deck, Four, Clubs));
        assert!(exists(&deck, Five, Clubs));
        assert!(exists(&deck, Six, Clubs));
        assert!(exists(&deck, Seven, Clubs));
        assert!(exists(&deck, Eight, Clubs));
        assert!(exists(&deck, Nine, Clubs));
        assert!(exists(&deck, Ten, Clubs));
        assert!(exists(&deck, Jack, Clubs));
        assert!(exists(&deck, Queen, Clubs));
        assert!(exists(&deck, King, Clubs));        
    }

    fn exists(deck: &Deck, rank: Rank, suit: Suit) -> bool {
        deck.cards.contains(&Card { rank, suit })
    }
}
