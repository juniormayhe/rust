use std::io::{stdin,stdout,Write};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // clear screen
    print!("{}[2J", 27 as char);

    let secret_number = rand::thread_rng().gen_range(0,101);

    loop
    {
        print!("Enter the number: ");
        let _= stdout().flush();

        let mut guess = String::new();
    
        stdin().read_line(&mut guess)
            .expect("failed to readline");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(numero) => numero,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{} too small compared to {}", guess, secret_number),
            Ordering::Greater =>println!("{} too big compared to {}", guess, secret_number),
            Ordering::Equal => { 
                println!("you win!");
                break;
            }
        }
    }
}
