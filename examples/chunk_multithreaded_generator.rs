use crossbeam_channel::unbounded;
use prime::generator::*;
use std::io::{self, BufWriter, Write};
use std::thread;

fn main() {
    let chunk_size = 1_000_000;
    // Asking for the number the user wants to arrive to.
    println!("Write the number you want to arrive to:");
    let mut max = String::new();
    io::stdin()
        .read_line(&mut max)
        .expect("Error reading input!");
    let max: u32 = max.trim().parse().expect("Error parsing input!");

    // Creating an output buffer and blocking stdout for faster printing.
    let stdout = io::stdout();
    let mut output_buffer = BufWriter::new(stdout.lock());

    let (tx, rx) = unbounded();

    thread::spawn(move || {
        par_prime_generator_map_chunks(
            max,
            Vec::new(),
            chunk_size,
            0,
            |_, _| true,
            |arr| {
                arr.iter().for_each(|&n| tx.send(n).unwrap());
            },
        );
    });

    for num in rx {
        writeln!(output_buffer, "{}", num).unwrap();
    }
}
