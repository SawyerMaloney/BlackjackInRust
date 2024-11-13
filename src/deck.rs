use crate::card::Card;
use crate::card::Suit;

use rand::prelude::*;

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

    pub fn shuffle(&mut self) {
        let mut shuffled: [usize; 52] = [0; 52];
        let mut shuffle_index = 0;
        while shuffle_index < 52 {
            shuffled[shuffle_index] = self.shuffle_next_card(&shuffled, shuffle_index);
            shuffle_index += 1;
        }
        self.cards = shuffled;
    }

    fn shuffle_next_card(&self, shuffled: &[usize; 52], shuffle_index: usize) -> usize {
        // use random to get an index that has not been used yet
        let mut next_card = self.random_card();
        while !self.card_not_within_index(&shuffled, shuffle_index, next_card) {
            next_card = self.random_card();
        }
        return next_card;
    }

    fn card_not_within_index(&self, shuffled: &[usize; 52], shuffle_index: usize, next_card: usize) -> bool {
        for i in 0..shuffle_index {
            if shuffled[i] == next_card {
                return false;
            }
        }
        return true;
    }

    fn random_card(&self) -> usize {
        let mut rng = rand::thread_rng();
        let ret: usize = rng.gen_range(0..52);
        return ret; // number one to 52
    }
}
