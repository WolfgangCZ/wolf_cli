use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn run() 
{
    loop
    {
        println!("Guess the number:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to reat the line");

        println!("You guessed {}", guess);
        let secret_number = rand::thread_rng().gen_range(0..=100);
        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too small, it was {secret_number} all along"),
            Ordering::Greater => println!("Too big, it was {secret_number} all along"),
            Ordering::Equal => println!("You win!"),
        }
    }
}

