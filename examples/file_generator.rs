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
    #[allow(clippy::uninit_vec)]
    let mut primes: Vec<u32> = unsafe {
        let len = file.metadata()?.len() as usize / std::mem::size_of::<u32>();
        let mut vec = Vec::with_capacity(len * 2);
        vec.set_len(len);
        file.read_exact(vec.align_to_mut::<u8>().1).unwrap();
        vec
    };

    let chunk_size: u32 = 10_000_000;

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
        |a, b| {
            //append_buffer.flush();
            if !running.load(Ordering::SeqCst) {
                return false;
            }
            println!("Calculating primes from {} to {}...", a, b);
            arrived_to = b;
            true
        },
        |arr| {
            append_buffer
                .write_all(unsafe { arr.align_to::<u8>() }.1)
                .unwrap()
        },
    );

    // Write where it arrived, to restart from there next time.
    append_buffer
        .write_all(arrived_to.to_ne_bytes().as_slice())
        .unwrap();

    println!("Done!");
    Ok(())
}
