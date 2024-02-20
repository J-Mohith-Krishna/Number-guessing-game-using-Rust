use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the Rust Number Guessing Game!");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    println!("I have generated a secret number between 1 and 100. Can you guess it?");
    
    loop {
        println!("Please input your guess:");
        
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        
        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("Congratulations! You guessed the secret number!");
                break;
            }
        }
    }
}
