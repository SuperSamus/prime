use crate::calculator::*;
use integer_sqrt::IntegerSquareRoot;
use rayon::prelude::*;
use std::cmp::{self, Ordering};

/// Calculates roughly the number of primes that are less than n.
pub fn gauss_function(n: u64) -> u64 {
    let x = n as f64;
    (x * 1.1055 / x.ln()) as u64
}

/// Ensures that the vector has 2 and 3 and maps them if they weren't there before.
fn get_basic_primes<F: Fn(u64) + Copy>(known_primes: &mut Vec<u64>, max: u64, found: F) {
    if known_primes.len() < 2 {
        *known_primes = [2, 3]
            .iter()
            .cloned()
            .filter(|n| *n < max)
            .map(|n| {
                found(n);
                n
            })
            .collect();
    };
}

/// Check if every number from the latest in your list to the max specified is prime and returns the primes found in a Vec.
///
/// The vector sent must be ordered. If you send an empty Vec, 2 and 3 will be added automatically.
///
/// # Examples
///
/// ```
/// use prime::generator::*;
/// 
/// // Start calculating from 3.
/// assert_eq!(prime_generator(20, Vec::new()), vec![2, 3, 5, 7, 11, 13, 17, 19]);
///
/// // Start calculating from 7, saving calculations.
/// assert_eq!(prime_generator(20, vec![2, 3, 5, 7]), vec![2, 3, 5, 7, 11, 13, 17, 19]);
///
/// // The Vec returned is the same if the last number of the Vec is greater than max.
/// assert_eq!(prime_generator(20, vec![2, 3, 5, 7, 11, 13, 17, 19, 23]), vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
/// ```
pub fn prime_generator(max: u64, known_primes: Vec<u64>) -> Vec<u64> {
    prime_generator_map(max, known_primes, |_| {})
}

/// Check if every number from the latest in your list to the max specified is prime and returns the primes found in a Vec.
///
/// You can specify the behavior when a prime is found. Primes already present in the list will not be mapped.
///
/// The vector sent must be ordered. If you send an empty Vec, 2 and 3 will be added automatically.
///
/// # Examples
///
/// ```
/// use prime::generator::*;
/// 
/// prime_generator_map(15, Vec::new(), |n| {println!("{}", n)}); // Prints 2, 3, 5, 7, 11, 13.
/// prime_generator_map(15, vec![2, 3, 5], |n| {println!("{}", n)}); // Prints 7, 11, 13.
/// prime_generator_map(15, vec![2, 3, 5, 7, 11, 13, 17, 19], |n| {println!("{}", n)}); // Prints nothing.
/// ```
pub fn prime_generator_map<F: Fn(u64) + Copy>(
    max: u64,
    mut known_primes: Vec<u64>,
    found: F,
) -> Vec<u64> {
    get_basic_primes(&mut known_primes, max, found);

    known_primes
        .reserve((gauss_function(max) as usize).saturating_sub(known_primes.capacity()));

    // +2 to not calculate the same number again. known_primes.last() is guaranteed to be odd.
    for n in ((known_primes.last().cloned().unwrap_or(1) + 2)..max).step_by(2) {
        if is_prime(n, known_primes.as_slice()) {
            known_primes.push(n);
            found(n);
        }
    }
    known_primes
}

/// Check if every number from the latest in your list to the max specified is prime by using parallel iteration for each number and returns the primes found in a Vec.
///
/// The vector sent must be ordered. If you send an empty Vec, 2 and 3 will be added automatically.
///
/// # Examples
///
/// ```
/// use prime::generator::*;
/// 
/// // Start calculating from 3.
/// assert_eq!(par_prime_generator(20, Vec::new()), vec![2, 3, 5, 7, 11, 13, 17, 19]);
///
/// // Start calculating from 7, saving calculations.
/// assert_eq!(par_prime_generator(20, vec![2, 3, 5, 7]), vec![2, 3, 5, 7, 11, 13, 17, 19]);
///
/// // The Vec returned is the same if the last number of the Vec is greater than max.
/// assert_eq!(par_prime_generator(20, vec![2, 3, 5, 7, 11, 13, 17, 19, 23]), vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
/// ```
pub fn par_prime_generator(max: u64, known_primes: Vec<u64>) -> Vec<u64> {
    par_prime_generator_map(max, known_primes, |_| {})
}

/// Check if every number from the latest in your list to the max specified is prime by using parallel iteration for each number and returns the primes found in a Vec.
///
/// You can specify the behavior when a prime is found. Primes already present in the list will not be mapped.
///
/// The vector sent must be ordered. If you send an empty Vec, 2 and 3 will be added automatically.
///
/// # Examples
///
/// ```
/// use prime::generator::*;
/// 
/// par_prime_generator_map(15, Vec::new(), |n| {println!("{}", n)}); // Prints 2, 3, 5, 7, 11, 13.
/// par_prime_generator_map(15, vec![2, 3, 5], |n| {println!("{}", n)}); // Prints 7, 11, 13.
/// par_prime_generator_map(15, vec![2, 3, 5, 7, 11, 13, 17, 19], |n| {println!("{}", n)}); // Prints nothing.
/// ```
pub fn par_prime_generator_map<F: Fn(u64) + Copy>(
    max: u64,
    mut known_primes: Vec<u64>,
    found: F,
) -> Vec<u64> {
    get_basic_primes(&mut known_primes, max, found);

    known_primes
        .reserve((gauss_function(max) as usize).saturating_sub(known_primes.capacity()));

    // +2 to not calculate the same number again. known_primes.last() is guaranteed to be odd.
    for n in (((known_primes.last().cloned().unwrap_or(1)) + 2)..max).step_by(2) {
        if par_is_prime(n, known_primes.as_slice()) {
            known_primes.push(n);
            found(n);
        }
    }
    known_primes
}

/// Check if every number from the latest in your list to the max specified is prime checking more numbers at the same time, and returns the primes found in a Vec.
///
/// Before checking more at a time though, only one at a time will be checked until the square root of max.
///
/// The vector sent must be ordered. If you send an empty Vec, 2 and 3 will be added automatically.
///
/// # Examples
///
/// ```
/// use prime::generator::*;
/// 
/// // Start calculating from 3.
/// assert_eq!(multi_prime_generator(20, Vec::new()), vec![2, 3, 5, 7, 11, 13, 17, 19]);
///
/// // Start calculating from 7, saving calculations.
/// assert_eq!(multi_prime_generator(20, vec![2, 3, 5, 7]), vec![2, 3, 5, 7, 11, 13, 17, 19]);
///
/// // The Vec returned is the same if the last number of the Vec is greater than max.
/// assert_eq!(multi_prime_generator(20, vec![2, 3, 5, 7, 11, 13, 17, 19, 23]), vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
/// ```
pub fn multi_prime_generator(max: u64, known_primes: Vec<u64>) -> Vec<u64> {
    multi_prime_generator_map(max, known_primes, |_| {})
}

/// Check if every number from the latest in your list to the max specified is prime checking more numbers at the same time, and returns the primes found in a Vec.
///
/// You can specify the behavior when a prime is found, but because they are checked in parallel they won't be mapped in order. Primes already present in the list will not be mapped.
///
/// Before checking more at a time though, only one at a time will be checked until the square root of max.
///
/// The vector sent must be ordered. If you send an empty Vec, 2 and 3 will be added automatically.
///
/// # Examples
///
/// ```
/// use prime::generator::*;
/// 
/// multi_prime_generator_map(15, Vec::new(), |n| {println!("{}", n)}); // Prints not in order 2, 3, 5, 7, 11, 13.
/// multi_prime_generator_map(15, vec![2, 3, 5], |n| {println!("{}", n)}); // Prints not in order 7, 11, 13.
/// multi_prime_generator_map(15, vec![2, 3, 5, 7, 11, 13, 17, 19], |n| {println!("{}", n)}); // Prints nothing.
/// ```
pub fn multi_prime_generator_map<F: Fn(u64) + Copy + Sync>(
    max: u64,
    mut known_primes: Vec<u64>,
    found: F,
) -> Vec<u64> {
    get_basic_primes(&mut known_primes, max, found);

    let sqr = max.integer_sqrt();

    // Beacause we can't modify the vector during the calculation, known_primes must be filled (single_threaded) until the square root of max so that we don't need to modify it.
    // However, if the vector is filled enough this step can be skipped and we can start from the last number in the vector.
    let last_prime = known_primes.last().cloned().unwrap_or(1);
    let start_from = match sqr.cmp(&last_prime) {
        Ordering::Less | Ordering::Equal => last_prime,
        Ordering::Greater => {
            known_primes = prime_generator_map(sqr, known_primes, found);
            sqr
        }
    };

    let new_primes = ((start_from + 1)..max)
        .into_par_iter()
        .filter(|n| n % 2 != 0 && is_prime(*n, known_primes.as_slice()))
        .map(|n| {
            found(n);
            n
        })
        .collect::<Vec<u64>>();
    known_primes.extend(new_primes);
    known_primes
}

/// Check if every number from the latest in your list to the max specified is prime checking more numbers at the same time, and returns the primes found in a Vec.
///
/// You can specify the behavior when a prime is found. The parallel iteration is divided in chunks, thus they will be mapped only once the chunk is complete. Primes already present in the list will not be mapped.
///
/// Before checking more at a time though, only one at a time will be checked until the square root of chunk_size.
///
/// The vector sent must be ordered. If you send an empty Vec, 2 and 3 will be added automatically.
///
/// # Examples
///
/// ```
/// use prime::generator::*;
/// 
/// multi_prime_generator_map_chunks(15, Vec::new(), 5, |n| {println!("{}", n)}); // Prints 2, 3, 5, 7, 11, 13.
/// multi_prime_generator_map_chunks(15, vec![2, 3, 5], 5, |n| {println!("{}", n)}); // Prints 7, 11, 13.
/// multi_prime_generator_map_chunks(15, vec![2, 3, 5, 7, 11, 13, 17, 19], 5, |n| {println!("{}", n)}); // Prints nothing.
/// ```
///
/// # Panics
///
/// The function panics if chunk_size is 0.
/// 
/// ```should_panic
/// use prime::generator::*;
/// 
/// multi_prime_generator_map_chunks(15, Vec::new(), 0, |n| {println!("{}", n)}); // Panics!
/// ```
pub fn multi_prime_generator_map_chunks<F: Fn(u64) + Copy + Sync>(
    max: u64,
    mut known_primes: Vec<u64>,
    chunk_size: usize,
    found: F,
) -> Vec<u64> {
    assert_ne!(chunk_size, 0);

    get_basic_primes(&mut known_primes, max, found);

    let sqr = chunk_size.integer_sqrt() as u64;

    // Beacause we can't modify the vector during the calculation, known_primes must be filled (single_threaded) until the square root of chunk so that we don't need to modify it.
    // However, if the vector is filled enough this step can be skipped and we can start from the last number in the vector.
    let last_prime = known_primes.last().cloned().unwrap_or(1);
    let start_from = match sqr.cmp(&last_prime) {
        Ordering::Less => last_prime,
        Ordering::Equal => sqr,
        Ordering::Greater => {
            known_primes = prime_generator_map(sqr, known_primes, found);
            sqr
        }
    };

    // +1 to not calculate the same number again. It's not +2 because it might start from the sqaure root,
    // which could be even, risking to skip a number (example: chunk_size = 20, sqr = 4, sqr + 2 == 6, 5 is skipped).
    for current_number in ((start_from + 1)..max).step_by(chunk_size) {
        let new_primes = (current_number..cmp::min(current_number + chunk_size as u64, max))
            .into_par_iter()
            .filter(|n| n % 2 != 0 && is_prime(*n, known_primes.as_slice()))
            .collect::<Vec<u64>>();

        for n in new_primes.iter() {
            found(*n);
        }
        known_primes.extend(new_primes);
    }
    known_primes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_test() {
        assert_eq!(prime_generator(0, Vec::new()), Vec::new());
        assert_eq!(
            prime_generator(20, Vec::new()),
            vec![2, 3, 5, 7, 11, 13, 17, 19]
        );
        assert_eq!(
            prime_generator(20, vec![2, 3, 5, 7]),
            vec![2, 3, 5, 7, 11, 13, 17, 19]
        );
        assert_eq!(
            prime_generator(20, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
    }

    #[test]
    fn par_generator_test() {
        assert_eq!(par_prime_generator(0, Vec::new()), Vec::new());
        assert_eq!(
            par_prime_generator(20, Vec::new()),
            vec![2, 3, 5, 7, 11, 13, 17, 19]
        );
        assert_eq!(
            par_prime_generator(20, vec![2, 3, 5, 7]),
            vec![2, 3, 5, 7, 11, 13, 17, 19]
        );
        assert_eq!(
            par_prime_generator(20, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
    }

    #[test]
    fn multi_generator_test() {
        assert_eq!(multi_prime_generator(0, Vec::new()), Vec::new());
        assert_eq!(
            multi_prime_generator(20, Vec::new()),
            vec![2, 3, 5, 7, 11, 13, 17, 19]
        );
        assert_eq!(
            multi_prime_generator(20, vec![2, 3, 5, 7]),
            vec![2, 3, 5, 7, 11, 13, 17, 19]
        );
        assert_eq!(
            multi_prime_generator(20, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
    }

    #[test]
    fn multi_chunks_generator_test() {
        assert_eq!(
            multi_prime_generator_map_chunks(0, Vec::new(), 5, |_| {}),
            Vec::new()
        );
        assert_eq!(
            multi_prime_generator_map_chunks(20, Vec::new(), 5, |_| {}),
            vec![2, 3, 5, 7, 11, 13, 17, 19]
        );
        assert_eq!(
            multi_prime_generator_map_chunks(20, vec![2, 3, 5, 7], 5, |_| {}),
            vec![2, 3, 5, 7, 11, 13, 17, 19]
        );
        assert_eq!(
            multi_prime_generator_map_chunks(
                20,
                vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29],
                5,
                |_| {}
            ),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
    }

    #[test]
    #[should_panic]
    fn multi_chunks_panic() {
        multi_prime_generator_map_chunks(20, Vec::new(), 0, |_| {});
    }
}
