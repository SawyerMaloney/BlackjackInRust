// Blackjack program
use std::io;
use std::io::Write;
mod card;

mod deck;
use deck::Deck;

mod hand;
use hand::Hand;


fn main() -> io::Result<()> {
    print!("Playing a simple game. Ready (y/n): ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if input.trim() == "y" {
        println!("Playing the game!");
        play_blackjack()?;
    } else {
        println!("Not playing the game");
    }

    Ok(())
}

fn play_blackjack() -> io::Result<()>{
    let mut deck = Deck::new();
    let mut balance: u32 = 100;
    let mut input = String::new();

    while input.trim() != "q" {
        input = String::new();
        deck.shuffle();

        let mut your_hand = setup_hand(&mut deck, false);
        let foe_hand = setup_hand(&mut deck, true);

        print!("Your balance: {}. How much would you like to bet (num or q to quit): ", balance); 
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        let int_input: u32 = match input.trim().parse::<u32>() {
            Ok(t) => t,
            Err(_) => 0,
        };

        if (int_input <= balance) & (input.trim() != "q") {
            let mut hit = String::new();
            let mut first_hand = true;
            while hit.trim() != "n" {
                if first_hand {
                    print_hand(&your_hand, &foe_hand);
                    first_hand = false;
                } else {
                    print!("\x1b[4F");
                    print_hand(&your_hand, &foe_hand);
                }
                print!("Do you want to hit (y/n): ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut hit)?;
                hit = hit;
                if hit.trim() == "y" {
                    your_hand.add_card(deck.draw_card());
                    hit = String::new();
                }
                if !your_hand.valid() {
                    println!("Bust!");
                    hit = String::from("n");
                }
            }
            println!("Your hand: {0}. Dealer hand: {1}", your_hand.to_string(), foe_hand.to_string());
            if your_hand.compare_hand(&foe_hand) {
                println!("You won: {}", input.trim());
                balance += int_input;
            } else {
                println!("You lost: {}", input.trim());
                balance -= int_input;
            }
        } else if input.trim() != "q" {
            println!("That bet is too big!");
        }
    }

    Ok(())
}

fn print_hand(your_hand: &Hand, dealer_hand: &Hand) {
    // print out the hand in format
    println!("-------------------------------------------");
    println!("Your hand | {} | Dealer showing | {} |", your_hand.to_string(), dealer_hand.show_dealer_hand());
    println!("-------------------------------------------");
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn setup_hand(deck: &mut Deck, is_foe: bool) -> Hand {
    let mut hand = Hand::new();
    if !is_foe {
        hand.add_card(deck.draw_card());
        hand.add_card(deck.draw_card());
    } else {
        while hand.value() < 17 {
            hand.add_card(deck.draw_card());
        }
    }
    return hand;
}
