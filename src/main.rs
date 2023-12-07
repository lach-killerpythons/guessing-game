use std::io;
use rand::Rng;

fn guess_game(answer: u32) -> bool{
    println!("guess the number");
    println!("your guess:");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    //println!("You guessed: {guess}")
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        _ => 0 // correct answer cannot be zero, but continue 
    };
        
    match guess {
        guess if guess == answer => {
            println!("you guessed {guess} - that is correct!");
            true
        }
        guess if guess < answer => {
            println!("you guessed {guess} - answer is larger");
            false
         }
        guess if guess > answer => {
            println!("you guessed {guess} - answer is smaller");
            false
        }
        _ => {
            println!("something went wrong!");
            false
        }        
    }
}

fn main() {
    let mut game_end = false;
    let secret_number = rand::thread_rng().gen_range(1,100);
    while !game_end {
        game_end = guess_game(secret_number);
    }
    
}
