use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    print!("Hello, friend - welcome to guess the number game!\n");
    print!("Try to guess my number :)\n\n");
    // generate random number -> 1-100
    let computer_number = rand::thread_rng().gen_range(1..101);

    // loop guess/input until correct guess
    loop {
        println!("Input your guess or press ctrl + C to end program/game");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // only matches numbers
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // catch any errors -> if input != number - guess again
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // break if correct
        // else -> give hint
        match guess.cmp(&computer_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
