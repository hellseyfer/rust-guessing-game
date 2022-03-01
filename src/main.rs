use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        /* read_line could fail, so it returns an enum Result wich can be Err or Ok. If returns err, the expect function will be executed*/
        io::stdin()
            .read_line(&mut guess)  // store user input to mutable variable 'guess'. & symbol means reference
            .expect("Failed to read line"); // if this instance of io::Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect
        

        /*
        The trim method on a String instance will eliminate any whitespace at the beginning and end. The trim method eliminates \n or \r\n
        the u32 type it’s a good default choice for a small positive number.
        the u32 annotation in this example program and the comparison with secret_number means that Rust will infer that secret_number should be a u32 as well
        */
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");  
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,     // The underscore, _, is a catchall value; in this example, we’re saying we want to match all Err values, no matter what information they have inside them.
        };


        println!("You guessed: {}", guess); // the {} prints variable values to the string

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}