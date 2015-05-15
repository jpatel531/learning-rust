extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop{
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line");

        println!("You guessed: {}",guess);

        let guess: u32 = match guess.trim().parse() { // returns an enum
            Ok(num) => num, // s'all good
            Err(_) => continue, // handle error
        };


        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>  {
                println!("You win!");
                break;
            },
        }        
    }



}
