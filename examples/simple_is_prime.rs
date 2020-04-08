use prime::calculator::*;
use std::io;

fn main() {
    // Asking for the number the user wants to check it's prime.
    println!("What number do you want to check it's prime?");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Error reading input!");
    let num: u64 = num.trim().parse().expect("Error parsing input!");

    match ignorant_is_prime(num, &[]) {
        true => {
            println!("The number {} is prime!", num);
        }
        false => {
            println!("The number {} is not prime. :(", num);
        }
    }
}
