use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess a number!");

    println!("Please enter your guessed number.");

    // Generate a secret number with a library crate , provided by rust team
    // there is an associated package in rand package called rng()
    // and in that rng(), we have a function named random_range which take a range
    //  where first digit is included and last digit is excluded in the range , e.g. 4..100
    // 4 included , 100 excluded
    // but if we want to include 100 to we can use '=' sign to include it.
    // 4..=100
    // 4 included , 100 included
    //
    // Note :- rand package generates pseudorandom , meaning they are not actually random , they
    // pretend to be random , in the computer there is nothing random
    // so they use time to make it half random.
    // they are called half random because if we make same situation of computer like same time ,
    // same temperature of cpu and more thing that regulate these no., we will get same random
    // everytime.
    let secret_number = rand::rng().random_range(0..=100);

    println!("{secret_number}");
    loop {
        let mut guess = String::new();

        // Handle error like below while taking input:
        /*
        match io::stdin().read_line(&mut guess) {
            Ok(bytes) => {
                // No need to print them , I was just handling the inputs and errors.
                /*
                println!("Number of bytes: {bytes}");
                println!("You entered : {guess}");
                */
            }
            Err(error) => {
                // Just print the errors
                println!("Failed to read the input.");
                println!("Error : {error}");
            }
        }
        */

        // Or Don't handle any error and throw it on terminal like this :
        // Better approach is to handle this yourself, but I won't be because if there
        // will be any error , it will likely from underlying operating system.
        //
        // expect() will the print the message given and panic the rust, or you can say produce error on
        //  terminal
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input.");

        println!("You guessed : {guess}");

        // We are shadowing the guess variable because we don't need it anymore.
        // it's like making a new variable of same name but with different properties and removing
        // the old ones.
        //  we are trimming it to remove the whitespace and '\n and \r' from the input so that we can
        //  parse this guess string into no and compare with secret number.
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid input.");
                continue;
            }
        };

        // Now just compare it with secret number
        // comparing has its arm of Ordering which compare two number using Ordering
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                continue;
            }
            Ordering::Greater => {
                println!("Too big!");
                continue;
            }
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
