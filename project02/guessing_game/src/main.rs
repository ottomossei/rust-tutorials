use rand::Rng;
use std::cmp::Ordering;
use std::io; //io input/output library into scope.

fn main() {
    // Generate random number
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        // Input your number
        println!("Guess the number");
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) //The & indicates that this argument is a reference,
            .expect("Failed to read line");
        println!("You guess: {}", guess);

        // "guess" is changed from String to Integer(u32) to compare(cmp) "secret_number"
        // "guess" is shadow(覆い隠す)ed.
        // trim() remove space both ends.(5\n → 5)
        // parse() on strings parses a string into some kind of number.
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
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
        }
    }
}
