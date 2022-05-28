// rust automatically brings in 'prelude' standard lib
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop    {

        println!("Please input your guess.");

        // use 'let' statement to create variable
        // variables are immutable by default in rust
        // let apples = 5; //immutable
        // add 'mut' to make it mutable
        let mut guess = String::new(); //mutable

        // 'String::new()' returns a new string instance, empty string
        // 'new' function is an associated function of the 'String' type
        // 'new' comes with manu types

        // we can also use 'std::io::stdin()' instead
        io::stdin()
            // '&' reference is immutable by default
            .read_line(&mut guess)
            // try to create new line when a method is called for readability
            // rust will warn if '.expect' is not used
            .expect("Failed to read line");

            // 'readline' return io::Result, type of enum
            // returns 'Ok' if success, 'Err' if failed

        let guess: u32 = match guess.trim().parse()   {
            Ok(num) => num,
            // '_' is a catchall value
            Err(_) => {
                println!("Please key in a valid number");
                continue;
                }
        };

        // {} is used as a placeholder
        println!("You guess: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
                }
        }
    }
}
    
    // cargo doc --open is useful