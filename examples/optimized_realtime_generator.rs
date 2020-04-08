use crossbeam_channel::unbounded;
use prime::generator::*;
use std::io::{self, BufWriter, Write};
use std::thread;

fn main() {
    // Asking for the number the user wants to arrive to.
    println!("Write the number you want to arrive to:");
    let mut max = String::new();
    io::stdin()
        .read_line(&mut max)
        .expect("Error reading input!");
    let max: u64 = max.trim().parse().expect("Error parsing input!");

    // Creating an output buffer and blocking stdout for faster printing.
    let stdout = io::stdout();
    let mut output_buffer = BufWriter::new(stdout.lock());

    let (tx, rx) = unbounded();

    thread::spawn(move || {
        prime_generator_map(max, Vec::new(), |n| {
            tx.send(n).unwrap();
        });
    });

    for n in rx {
        writeln!(output_buffer, "{}", n).unwrap();
    }
}
