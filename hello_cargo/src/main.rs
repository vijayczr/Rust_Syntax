use std::io;
//use std::cmp::Ordering;
use rand::prelude::*;
//use rand::Rng;

fn main() {

     let secret_number = rand::rng().random_range(1..=10);
// //  let secret_number = rand::thread_rng().gen_range(1..=100);

 //   println!("your secret number is {}" , secret_number);
    println!("Guess the number");




    loop {
        let mut guess = String::new();
        println!("Please enter your number!");
        io::stdin().read_line(&mut  guess).expect("there is some issue");

        println!("your guess is {}" , guess);
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
            println!("Please enter a valid number!");
            continue;
        }
        };
    
    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("your number is less that secret number!"),
    //         Ordering::Greater => println!("your number is more than secret number!"),
    //         Ordering::Equal => println!("your guess is correct!")
    
    //    }
    
        if guess < secret_number {
            println!("your number is less that secret number!");
        }else if guess > secret_number {
            println!("your number is more than secret number!");
        }else{
            println!("your guess is correct!"); 
            break;
        }
    }






        // --snip--

        // let mut guess = String::new();

        // io::stdin()
        //     .read_line(&mut guess)
        //     .expect("Failed to read line");
    
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
        // println!("You guessed: {guess}");
    
        // match guess.cmp(&secret_number) {
        //     Ordering::Less => println!("Too small!"),
        //     Ordering::Greater => println!("Too big!"),
        //     Ordering::Equal => println!("You win!"),
        // }
    

}
