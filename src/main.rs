// Blackjack program
use std::io;
use std::io::Write;
mod card;

mod deck;
use deck::Deck;

mod hand;
use hand::HandRanks;
use hand::Hand;


fn main() -> io::Result<()> {
    print!("Playing a simple game. Ready (y/n): ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if input.trim() == "y" {
        println!("Playing the game!");
        play_the_game()?;
    } else {
        println!("Not playing the game");
    }

    Ok(())
}

fn play_the_game() -> io::Result<()>{
    let mut deck = Deck::new();
    let mut balance: u32 = 100;
    let mut input = String::new();

    while input.trim() != "q" {
        input = String::new();
        deck.shuffle();
        let mut your_hand = Hand::new();
        your_hand.add_card(deck.draw_card());
        print!("Your hand: {0}. Current balance: {1}. How much would you like to bet? (num or q to quit): ", your_hand.to_string(), balance);
        let mut foe_hand = Hand::new();
        foe_hand.add_card(deck.draw_card());
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        let int_input: u32 = match input.trim().parse::<u32>() {
            Ok(t) => t,
            Err(_) => 0,
        };

        if int_input <= balance {
            println!("Your hand: {0}. Their hand: {1}", your_hand.to_string(), foe_hand.to_string());
            if your_hand.compare_hand(&foe_hand) {
                println!("You won: {}", input.trim());
                balance += int_input;
            } else {
                println!("You lost: {}", input.trim());
                balance -= int_input;
            }
        } else {
            println!("That bet is too big!");
        }
    }

    Ok(())
}
