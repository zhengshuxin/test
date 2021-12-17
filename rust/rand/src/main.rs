use std::io;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    let secret_num = rand::thread_rng().gen_range(1, 100);
    println!("The secret_num is {}", secret_num);

    println!("Please guess number:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("guess {}", guess);
}
