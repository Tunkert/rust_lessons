use std::io; // input/output library from the standard library
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    loop {
        println!("Enter a number, bro: "); // a macro that prints to the screen
        let mut guess = String::new(); // create a variable that is mutable, type string

        let secret_number = rand::thread_rng().gen_range(1..=10); // defaults to type i32

        /*
    rust variables are immutable by default
    String::new() creates a new instance of type string from the standard library
    that is a growable, UTF-8 encoded bit of text
    :: indicates new is associated with String
     */

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // will crash program if error occurs, more about errors later

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number, you fucker!");
                continue;
            },
        };

        /*
    io::stdin() could be written as std::io::stdin() if we didn't import the io library from
    the standard library
    & is a reference to the variable without having to create multiple copies in memory
     */

        print!("You guessed {guess}\n"); // a place holder
        print!("The secret number was {secret_number}.\n");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small, bro."),
            Ordering::Greater => println!("Too large, bro."),
            Ordering::Equal => {
                println!("Just right, bro!");
                break;
            },
        }
    }
    /*
    types will not match ... yet
     */
}
