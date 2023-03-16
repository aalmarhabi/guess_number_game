use rand::prelude::*;
use std::io;

// main function to guess random number

fn main() {
    // generate random between 1 to 100
    let rand_num = rand::thread_rng().gen_range(1..101);
    // make count of how many trial user had
    let mut count_trial = 0;
    // you can use while true {} too here
    loop {
        // create buffer to hold user input
        let mut buffer = String::new();
        println!("Enter your guess?!: ");
        io::stdin().read_line(&mut buffer);
        // parse buffer to int i32
        let guess: i32 = buffer.trim().parse().unwrap(); // use unwrap to ignore error handling in enum

        if rand_num > guess {
            println!("\n{} Your guess is small", guess);
            count_trial += 1;
        } else if rand_num < guess {
            println!("\n{} Your guess is big", guess);
            count_trial += 1;
        } else {
            println!("\nYour guess is correct, the number was {}", rand_num);
            if count_trial != 6 {
                println!(
                    "You guessed the number in {} trial. Therefore you did not when try again!",
                    count_trial
                );
            } else {
                println!(
                    "You guessed the number in {} trial. You did it! WIN",
                    count_trial
                );
            }
            break;
        }
    }
}
