use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let number_to_guess = rand::thread_rng().gen_range(0..=100);

    loop {
        // println!("The number to guess is {number_to_guess}");

        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("You guessed: {}\n", guess);

        match guess.cmp(&number_to_guess) {
            Ordering::Less => println!("The number is higher!"),
            Ordering::Greater => println!("The number is lower!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
