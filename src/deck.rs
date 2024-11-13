use crate::card::Card;
use crate::card::Suit;

pub struct Deck {
    cards: [usize; 52], // index of each card
    index: usize,
}

impl Deck {
    pub fn new() -> Self {
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

    pub fn draw_card(&mut self) -> Card {
        let int_card: usize = self.draw_int_card();
        self.convert_int_to_card(int_card)
    }
}
