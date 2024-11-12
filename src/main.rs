// Blackjack program

enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

struct Card {
    suit: Suit,
    value: u8,
}

impl Card {
    fn new(suit: Suit, value: u8) -> Self {
        Self {
            suit,
            value,
        } 
    }

    fn suit_to_string(&self) -> String {
        match self.suit {
            Suit::Clubs => String::from("♣"),
            Suit::Diamonds => String::from("♦"),
            Suit::Hearts => String::from("♥"),
            Suit::Spades => String::from("♠"),
        }
    }

    fn value_to_string(&self) -> String {
        if self.value < 11 {
            return String::from(self.value.to_string());
        }
        self.value.to_string()
    }

    fn to_string(&self) -> String {
        format!("{0}{1}", self.value_to_string(), self.suit_to_string())
    }

    fn suit_to_int(&self) -> u8 {
        match self.suit {
            Suit::Clubs => 0,
            Suit::Diamonds => 1,
            Suit::Hearts => 2,
            Suit::Spades => 3,
        }
    }

    // returns true if greater, false if not
    fn is_greater(&self, card: &Card) -> bool {
        if self.value > card.value {
            return true;
        }
        else if self.value < card.value {
            return false;
        }
        else {
            // same value, so compare suit
            // putting >= because tie goes to user
            self.suit_to_int() >= card.suit_to_int()
        }
    }
}

struct Deck {
    cards: [u8; 52], // index of each card
}

impl Deck {
    fn new() -> Self {
        Self { 
            cards: [0..52],
    }
}

fn main() {
    let c = Card::new(Suit::Spades, 7);
    let b = Card::new(Suit::Hearts, 7);
    println!("{}", c.to_string());
    if c.is_greater(&b) {
        println!("{0} is greater than {1}", c.to_string(), b.to_string());
    }
    else if b.is_greater(&c) {
        println!("{0} is greater than {1}", b.to_string(), c.to_string());
    }
}
