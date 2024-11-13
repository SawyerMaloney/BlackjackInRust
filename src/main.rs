// Blackjack program
mod card;
use card::Card;
use card::Suit;

mod deck;
use deck::Deck;


fn main() {
    let mut deck: Deck = Deck::new();

    let card: Card = deck.draw_card();
    println!("Draw card: {}", card.to_string());
}
