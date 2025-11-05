use std::io::stdin;

fn main(){
    const PI: f64 = 3.14;
    let mut inputa = String::new();
    println!("Enter the first numbers:");
    stdin().read_line(&mut inputa).expect("failed to read line");

    let mut inputb = String::new();
    println!("Enter the second numbers:");
    stdin().read_line(&mut inputb).expect("failed to read line");

    let a: i32 = inputa.trim().parse().expect("invalid input");
    let b: i32 = inputb.trim().parse().expect("invalid input");

    let sum = a + b;
    println!("{}", sum);
}