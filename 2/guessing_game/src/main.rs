// Include used libraries.
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// First function being executed by rust.
fn main() {

    // Generate a random number (start..=end)
    let random_number = rand::thread_rng().gen_range(1..=100);
    loop {
        // Create a mutable variable named string which is of type string.
        // A string is UTF-8 encoded.
        // :: is an associated function
        let mut guess = String::new();

        // Println macro.
        println!("Guess a number:");

        //std::io::stdin()
        io::stdin()

            // references are immutable as well, so need mut keyword to pass guess. 
            // Function binds user input to guess variable.
            .read_line(&mut guess)
            .expect("Failed to get guess");

        // Cast the string into a number. May result into a faulty cast,
        // so by using a match, you can handle the error.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Compare guess to random number
        match guess.cmp(&random_number){

            // Match looks at the condition.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}