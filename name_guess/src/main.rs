use std::io;

fn main(){
    println!("Welcome to guess name game");
    let name = "Marvellous".to_lowercase();

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read user input");

    if user_input.trim().to_lowercase() == name {
        println!("Your guess is correct!");
    }else {
        println!("wrong guess, try again!");
    }
}