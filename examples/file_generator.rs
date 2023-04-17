use prime::generator::*;
use std::fs::OpenOptions;
use std::io::{prelude::*, BufWriter};
use std::mem::size_of;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() -> std::io::Result<()> {
    let path_arg = std::env::args().nth(1);
    let path: &str = if let Some(p) = path_arg.as_ref() {
        p
    } else {
        "primes.dat"
    };

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
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(path)?;

    // Importing current primes from the file. If the file didn't exist or is too short, clear everything and calculate until n_before_multithreading.
    println!("Reading file...");
    let mut primes: Vec<u8> = Vec::new();
    file.read_to_end(&mut primes)?;

    let mut primes = unsafe {
        Vec::from_raw_parts(
            primes.as_mut_ptr() as *mut u32,
            primes.len() / std::mem::size_of::<u32>(),
            primes.capacity() / std::mem::size_of::<u32>(),
        )
    };

    let chunk_size: u32 = 1_000_000;

    // The last number is not a prime, but where the program arrived to last time
    let start_from = if let Some(s) = primes.pop() {
        file.seek(std::io::SeekFrom::End(
            -i64::try_from(size_of::<u32>()).unwrap(),
        ))?;
        s
    } else {
        0
    };

    let mut append_buffer = BufWriter::new(file);

    // Calculating new primes!
    println!("Time to start calculating! Stop by pressing Ctrl + C.");

    let mut arrived_to = 0;

    par_prime_generator_map_chunks(
        u32::MAX,
        primes,
        start_from,
        chunk_size,
        |n| append_buffer.write_all(n.to_ne_bytes().as_slice()).unwrap(),
        |a, b| {
            // append_buffer.flush();
            if !running.load(Ordering::SeqCst) {
                return false;
            }
            println!("Calculating primes from {} to {}...", a, b);
            arrived_to = b;
            true
        },
    );

    // Write where it arrived, to restart from there next time.
    append_buffer
        .write_all(arrived_to.to_ne_bytes().as_slice())
        .unwrap();

    println!("Done!");
    Ok(())
}
