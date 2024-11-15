use crate::card::Card;
use crate::card::Suit;


/*
 *  Implementation of a hand. Idea: program draws cards from the deck and adds them to one of two.
 *  Compare function compares them for easy code
 *
 * */


pub struct Hand {
    cards: [Card; 52],
    num_of_cards: usize,
}

impl Hand {
    pub fn new() -> Self {
        return Hand {
            cards: [Card::new(Suit::Diamonds, 0); 52],
            num_of_cards: 0,
        };
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards[self.num_of_cards] = card;
        self.num_of_cards += 1;
    }

    pub fn to_string(&self) -> String {
        let mut ret_string = String::new();
        for i in 0..self.num_of_cards {
            ret_string.push_str(&self.cards[i].to_string());
            ret_string.push_str(" ");
        }
        ret_string
    }

    pub fn compare_hand(&self, hand: &Hand) -> bool {
        self.valid() & ((self.value() > hand.value()) | !hand.valid())
    }

    pub fn value(&self) -> usize {
        let mut sum = 0;
        for card in 0..self.num_of_cards {
            sum += self.cards[card].value(); 
        }
        sum
    }

    pub fn valid(&self) -> bool {
        (self.value() < 22) & (self.num_of_cards != 0)
    }

    pub fn show_dealer_hand(&self) -> String {
        return self.cards[0].to_string();
    }
}
