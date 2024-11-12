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
}

fn main() {
    let c = Card::new(Suit::Clubs, 7);
    println!("{}", c.to_string());
}
