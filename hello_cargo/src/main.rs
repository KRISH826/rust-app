use rand::prelude::*;
use std::io;

fn guess_gaming() {
    println!("Guess the number!");

    println!("Please input your guess:");

    let secret_number = rand::rng().random_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        let mut guess2 = String::new();

        io::stdin()
            .read_line(&mut guess2)
            .expect("failed to read line");

        let guess_num: u32 = match guess2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };

        println!("Your guess is: {guess_num}");

        match guess_num.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("you are near to the number!"),
            std::cmp::Ordering::Greater => println!("you are far from the number!"),
            std::cmp::Ordering::Equal => {
                println!("you guessed the number!");
                break;
            },
        }
    }
}

fn age_game() {
    let age = 20;
    println!("my age is {age}");
}

fn ruthless_guy(name: String) {
    println!("I am a ruthless guy named {name}");
}

fn main() {
    age_game();
    guess_gaming();
    ruthless_guy("Alice".into());
}
