// Blackjack program
mod card;

mod deck;
use deck::Deck;


fn main() {
    let mut deck = Deck::new();
    let card = deck.draw_card();
    println!("Card: {}", card.to_string());
    println!("Shuffling....");
    deck.shuffle();
    for _i in 0..10 {
        let new_card = deck.draw_card();
        println!("New drawn card: {}", new_card.to_string());
    }
    println!("Drawing two cards and comparing them");
    let card_1 = deck.draw_card();
    let card_2 = deck.draw_card();
    println!("Card 1: {0}, Card 2: {1}", card_1.to_string(), card_2.to_string());
    if card_1.is_greater(&card_2) {
        println!("Card 1 is greater than card 2");
    }
    else if card_2.is_greater(&card_1) {
        println!("Card 2 is greater than card 1");
    } else {
        println!("Something has gone wrong");
    }
}
