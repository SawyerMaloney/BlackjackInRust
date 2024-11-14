#[derive(Copy, Clone)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Copy, Clone)]
pub struct Card {
    suit: Suit,
    value: usize,
}

impl Card {
    pub fn new(suit: Suit, value: usize) -> Self {
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
        } else if self.value == 11 {
            return String::from("Jack");
        } else if self.value == 12 {
            return String::from("Queen");
        }
        return String::from("King");
    }

    pub fn to_string(&self) -> String {
        format!("{0}{1}", self.value_to_string(), self.suit_to_string())
    }

    fn suit_to_int(&self) -> usize {
        match self.suit {
            Suit::Clubs => 0,
            Suit::Diamonds => 1,
            Suit::Hearts => 2,
            Suit::Spades => 3,
        }
    }

    pub fn value(&self) -> usize {
        self.value
    }

    // returns true if greater, false if not
    pub fn is_greater(&self, card: &Card) -> bool {
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
