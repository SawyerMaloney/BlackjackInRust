// Blackjack program

enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

struct Card {
    suit: Suit,
    value: usize,
}

impl Card {
    fn new(suit: Suit, value: usize) -> Self {
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

    fn suit_to_int(&self) -> usize {
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
    cards: [usize; 52], // index of each card
    index: usize,
}

impl Deck {
    fn new() -> Self {
        let mut a :[usize; 52] = [0; 52];
        for i in 0..52 {
            a[i] = i;
        }

        Self { 
            cards: a,
            index: 0,
        }
    }

    // returns a created Card struct based on usize int drawn from the deck
    fn convert_int_to_card(&self, mut card: usize) -> Card {
        // convert into suit (int) and card (int)
        let mut suit: usize = 0;
        while card > 12 {
            suit += 1;
            card -= 12;
        }
        // increase card by one because we're 0 indexing
        card += 1;
        let suit: Suit = self.int_to_suit(suit);
        return Card::new(suit, card);
    }

    fn int_to_suit(&self, suit: usize) -> Suit {
        match suit {
            0 => Suit::Clubs,
            1 => Suit::Diamonds,
            2 => Suit::Hearts,
            3 => Suit::Spades,
            _ => Suit::Clubs,
        }
    }

    fn draw_int_card(&mut self) -> usize {
        let ret = self.cards[self.index];
        self.index += 1;
        return ret;
    }

    fn draw_card(&mut self) -> Card {
        let int_card: usize = self.draw_int_card();
        self.convert_int_to_card(int_card)
    }
}

fn main() {
    let mut deck: Deck = Deck::new();

    let card: Card = deck.draw_card();
    println!("Draw card: {}", card.to_string());
}
