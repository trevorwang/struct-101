use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");
    let mut rng = thread_rng();
    let secret_number = rng.gen_range(0..100);
    // println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        let g = Guess::new(guess);

        match g.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

struct Guess {
    value: u32,
}
impl Guess {
    fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
    fn value(&self) -> u32 {
        self.value
    }
}
