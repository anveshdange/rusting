// Modular imports 
use std::io;
use rand::Rng; // Bringing range crait to scope
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess the number!!");
    let secret_number = rand::thread_rng()
                                .gen_range(1..101);
    println!("The number is: {}", secret_number);
    
    loop{
        println!("Please input your guess: ");
        let mut guess = String::new() ;

        io::stdin()
            .read_line(&mut guess) 
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num ,
            Err(_) => {
                println!("Please Input a number NOT STRING!!") ;
                continue;
            },
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}","You win!".green().bold());
                break;
            },
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Less => println!("{}","Too small".red()),
        }
    }
    
}
