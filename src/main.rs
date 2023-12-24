use std::{ io, cmp::Ordering };
use rand::Rng;
fn main() {
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..10);
    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");
        println!("You guessed: {}", guess);

        let guess: u32 = guess.trim().parse().expect("please use a number");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The Value Is Smaller Than The Original Value"),
            Ordering::Greater => println!("The Value Is Greater Than The Original Value"),
            Ordering::Equal => {
                println!("Congrats thats the value");
                break;
            }
        }
    }
}
