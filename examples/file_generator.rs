use integer_sqrt::IntegerSquareRoot;
use prime::{calculator::*, generator::*};
use rayon::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;


fn main() -> std::io::Result<()> {
    let chunk_size: usize = 1_000_000;
    // Numbers to do singlethreaded before going multithread. *2 is to be sure it's high enough.
    let n_before_multithreading = chunk_size.integer_sqrt() * 2;
    let path: &str = "primes.txt";

    // Setting graceful closing.
    println!("Setting things up...");
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("Ctrl + C pressed. Exiting...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Read the file and store its contents in a Vec.
    println!("Creating or opening the file {}...", path);
    let file = OpenOptions::new().create(true).write(true).read(true).open(path)?;

    // Importing current primes from the file. If The file didn't exist or is too short, clear everything and calculate until n_before_multithreading.
    println!("Reading file...");
    let read_buffer = BufReader::new(file);
    let n_lines = read_buffer.lines().count();

    let mut primes: Vec<u64> = match n_lines < gauss_function(n_before_multithreading as u64) as usize {
        true => {
            println!("Generating the first primes...");
            let file = File::create(path)?;
            let mut write_buffer = BufWriter::new(file);
            let primes = prime_generator(n_before_multithreading as u64, Vec::new());
            for n in primes.iter() {
                writeln!(write_buffer, "{}", n)?;
            }
            write_buffer.flush()?;
            primes
        }
        false => {
            println!("Importing primes...");
            let mut primes = Vec::with_capacity(n_lines);
            let file = File::open(path)?;
            let read_buffer = BufReader::new(file);
            for line in read_buffer.lines() {
                primes.push(line?.parse().expect("Error: the file has a line without a number!"));
            }
            primes
        }
    };


    // Calculating new primes!
    println!("Time to start calculating! Stop by pressing Ctrl + C.");
    let file = OpenOptions::new().append(true).open(path)?;
    let mut append_buffer = BufWriter::new(file);

    let mut current_number = *primes.last().unwrap();
    while running.load(Ordering::SeqCst) {
        let last_number = current_number.saturating_add(chunk_size as u64);
        println!("Calculating primes from {} to {}...", current_number, last_number - 1);
        let new_primes = (current_number..last_number)
            .into_par_iter()
            .filter(|n| n % 2 != 0 && is_prime(*n, primes.as_slice()))
            .collect::<Vec<u64>>();

        primes.extend(new_primes.into_iter().map(|n| {
            writeln!(append_buffer, "{}", n).unwrap();
            n
        }));

        append_buffer.flush()?;
        current_number += chunk_size as u64;
    }

    println!("Done!");
    Ok(())
}
