use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing game!");

    let game = rand::thread_rng().gen_range(1..=20);

    loop {
        let mut guess = String::new();
        io::stdin()
                .read_line(&mut guess)
                .expect("Couldn't Read Data");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}");

        match guess.cmp(&game){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Rock!!!");
                break;
            }
        }
    }
}