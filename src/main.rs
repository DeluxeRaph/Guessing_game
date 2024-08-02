use std::cmp::Ordering;
use std::io;

// We want 2 players in this guessing game.
//  Player 1 to set the number
//  Player 2 to guess the number

fn main() {
    println!("Guess the number!");

    let mut secret_number = Default::default();

    io::stdin()
        .read_line(&mut secret_number)
        .expect("Not a number");

        let secret_number: u32 = match secret_number.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Problem opening the file:"),
        };


    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read linne");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
