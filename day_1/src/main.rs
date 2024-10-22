use rand::Rng;
//compparing the two things / numbers
use std::cmp::Ordering;
use std::io::{self, Write};
use colored::*;

fn main() {
    println!("Guess The Number !!");

    //high rang is exclusive
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is : {}", secret_number);

    loop {
        print!("Please Input Your Guess : ");

        let _ = std::io::stdout().flush();

        //By default its immutable (means can't change further)
        // so we have to add the mut keyword
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed To Read Line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You Guessed : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too Small!!".red()),
            Ordering::Greater => println!("{}", "Too Big".red()),
            Ordering::Equal => {
                println!("{}","You Win!!".green());
                break;
            },
            
        }
    }
}
