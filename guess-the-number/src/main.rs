use rand::Rng;
use std::cmp::Ordering;
use std::time::Instant;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..1_000_000);
    let mut guess = String::new();
    let mut tries = 0;
    let timer = Instant::now();
    println!("Guess a number between 1 and 1 million!");
    loop {
        tries += 1;
        guess.clear();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!(
                    "You win after {} tries and {} seconds!",
                    tries,
                    timer.elapsed().as_secs()
                );
                break;
            }
        }
    }
}
