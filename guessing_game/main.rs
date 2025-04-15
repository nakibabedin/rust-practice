use std::io;
use rand::Rng;

fn main() {
    
    let mut rng = rand::thread_rng();
    let mut guess_str = String::new();
    let target_num :i32 = rng.gen_range(1..=10);
    
    loop{
        println!("Guess a number between 1 and 10");

        io::stdin().read_line(&mut guess_str).expect("Failed to read your guess");
        let guess_int : i32 = guess_str.trim().parse().expect("Please enter a valid number");
        
        if guess_int == target_num {
            break;
        }

        println!("Try again!");
        guess_str.clear();
    }

    println!("Game over!");    
    
}
