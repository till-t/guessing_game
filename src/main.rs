use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Hello...");
    println!("Lets play a guessing game!");
    println!("The number is in between 1 and 100..");
    println!("The secret number is: {}", secret_number);

    loop {
        let mut guess = String::new();

        println!("Enter a guess: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("Your guess was: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            } 
        }
    }
    

}
