use prime::generator::*;
use std::io;

fn main() {
    // Asking for the number the user wants to arrive to.
    println!("Write the number you want to arrive to:");
    let mut max = String::new();
    io::stdin()
        .read_line(&mut max)
        .expect("Error reading input!");
    let max: u64 = max.trim().parse().expect("Error parsing input!");

    prime_generator(max, Vec::new()).iter().for_each(|n| {
        println!("{}", n);
    });
}
