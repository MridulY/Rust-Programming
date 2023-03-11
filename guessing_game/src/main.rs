use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Your secret number is: {secret_number}");
loop{
    let mut guess = String::new();
    //taking input from user
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed");
    //using shadow to reuse the guess variable again
    //converting the guess from string to integer
    //trim() removes white space
    //parse() convert string to integer
    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };
    //after curly braces we using these to handle error

    println!("You guessed: {guess}");
    //comparing the value of guess with secret number
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Low Number"),
        Ordering::Equal => {
            println!("You Win");
            break;
        }
        Ordering::Greater => println!("High Number"),
    }
}
}