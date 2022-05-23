use std::io;

fn main() {
    println!("Hello...");
    println!("Lets play a guessing game!");
    println!("The number is in betwee 1 and 100..");
    println!("Enter your first guess: ");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guess was: {}", guess);

}
