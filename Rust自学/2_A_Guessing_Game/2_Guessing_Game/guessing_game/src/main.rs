use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop{
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("[Please input a digit type!]");
                continue;
            },
        };

        println!("You guess: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!{"You win!"};
                break;
            }
        }
    }
}
