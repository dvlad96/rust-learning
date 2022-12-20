use std::io; /* Input/Output library */
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    /* While loop */
    loop {
        /* Create a variable to store user input
           By deafault variables are immutable so we need to
           declare our variable to be mutable using the mut keyword  */
        let mut guess = String::new(); /* We have created a mutable variable that 
                                          is currently bound to a new, empty instance of a String */

        /* The stdin function from the io module, which will allow us to handle user input */
        io::stdin()
        .read_line(&mut guess) /* Like variables, references are immutable by default */
        .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
                }
            };

        println!("You guessed: {guess}");
        println!("The number was: {secret_number}");
    }

}
