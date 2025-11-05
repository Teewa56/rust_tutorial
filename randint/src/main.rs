use std::io::stdin;
use rand::Rng;

fn main(){
    println!("Guess the number");

    let number = rand::thread_rng().gen_range(1..100);

    let mut guess = String::new();

    stdin()
        .read_line(&mut guess)
        .expect("Failed to reada line");

    println!("You guessed: {}", guess);
    
    if guess.trim() == number.to_string() {
        println!("You win!");
    } else {
        println!("You lose! The number was: {}", number);
    }
}