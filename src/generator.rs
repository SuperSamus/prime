use crate::calculator::*;
use num::integer::Roots;
use num::{range_step, CheckedAdd, FromPrimitive, Integer, ToPrimitive};
use rayon::prelude::*;
use std::cmp;

/// Calculates roughly the number of primes that are less than n.
///
/// The approximated result is always greater than the actual one
pub fn gauss_function<N: Integer + FromPrimitive + ToPrimitive>(n: N) -> N {
    // https://mathworld.wolfram.com/PrimeNumberTheorem.html
    let x = n.to_f64().unwrap();
    const LEGENDRE_CONSTANT: f64 = -1.08366;
    N::from_f64((x / (x.ln() + LEGENDRE_CONSTANT)).ceil()).unwrap()
}

/// Ensures that the vector has 2 and 3 and maps them if they weren't there before.
fn get_basic_primes<N: Roots + Copy, F: FnMut(N)>(
    known_primes: &mut Vec<N>,
    until: N,
    mut found: F,
) {
    known_primes.extend(
        [N::one() + N::one(), N::one() + N::one() + N::one()]
            .into_iter()
            .skip(known_primes.len())
            .filter(|n| *n < until)
            .inspect(|n| found(*n)),
    )
}

/// Check if every number from the start to the end specified is prime and returns the primes found in a Vec.
///
/// The latest prime in the list list is used as a starting point if it is higher than the start supplied.
///
/// The Vec sent must be ordered. If you send an empty Vec, 2 and 3 will be added automatically.
///
/// # Examples
///
/// ```
/// use prime::generator::*;
///
/// // Start calculating from 3.
/// assert_eq!(prime_generator(20, Vec::new(), 0), vec![2, 3, 5, 7, 11, 13, 17, 19]);
///
/// // Start calculating from 7, saving calculations.
/// assert_eq!(prime_generator(20, vec![2, 3, 5, 7], 0), vec![2, 3, 5, 7, 11, 13, 17, 19]);
///
/// // The Vec returned is the same if the last number of the Vec is greater than max.
/// assert_eq!(prime_generator(20, vec![2, 3, 5, 7, 11, 13, 17, 19, 23], 0), vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
/// ```
pub fn prime_generator<N: Roots + FromPrimitive + ToPrimitive + Copy + CheckedAdd>(
    until: N,
    known_primes: Vec<N>,
    start_from: N,
) -> Vec<N> {
    prime_generator_map(until, known_primes, start_from, |_| {})
}

/// Check if every number from the start to the end specified is prime and returns the primes found in a Vec.
///
/// The latest prime in the list list is used as a starting point if it is higher than the start supplied.
///
/// You can specify the behavior when a prime is found. Primes already present in the list will not be mapped.
///
/// The Vec sent must be ordered. If you send an empty Vec, 2 and 3 will be added automatically.
///
/// # Examples
///
/// ```
/// use prime::generator::*;
///
/// prime_generator_map(15, Vec::new(), 0, |n| {println!("{}", n)}); // Prints 2, 3, 5, 7, 11, 13.
/// prime_generator_map(15, vec![2, 3, 5], 0, |n| {println!("{}", n)}); // Prints 7, 11, 13.
/// prime_generator_map(15, vec![2, 3, 5, 7, 11, 13, 17, 19], 0, |n| {println!("{}", n)}); // Prints nothing.
/// ```
pub fn prime_generator_map<
    N: Roots + FromPrimitive + ToPrimitive + Copy + CheckedAdd,
    F: FnMut(N),
>(
    until: N,
    mut known_primes: Vec<N>,
    start_from: N,
    mut found: F,
) -> Vec<N> {
    get_basic_primes(&mut known_primes, until, &mut found);

    known_primes.reserve(
        (gauss_function(until).to_usize().unwrap()).saturating_sub(known_primes.capacity()),
    );

    let two = N::one() + N::one();

    let start_from = start_from.max(known_primes.last().cloned().unwrap_or(N::zero()) + two);

    // +2 to not calculate the same number again. known_primes.last() is guaranteed to be odd.
    for n in range_step(start_from, until, two) {
        if is_prime(n, known_primes.as_slice()) {
            known_primes.push(n);
            found(n);
        }
    }
    known_primes
}

/// Parallelely check if every number from the start to the end specified is prime and returns the primes found in a Vec.
///
/// The latest prime in the list list is used as a starting point if it is higher than the start supplied.
///
/// Before checking in parallel, only one number at a time will be checked until the square root of max.
///
/// The Vec sent must be ordered. If you send an empty Vec, 2 and 3 will be added automatically.
///
/// # Examples
///
/// ```
/// use prime::generator::*;
///
/// // Start calculating from 3.
/// assert_eq!(par_prime_generator(20, Vec::new(), 0), vec![2, 3, 5, 7, 11, 13, 17, 19]);
///
/// // Start calculating from 7, saving calculations.
/// assert_eq!(par_prime_generator(20, vec![2, 3, 5, 7], 0), vec![2, 3, 5, 7, 11, 13, 17, 19]);
///
/// // The Vec returned is the same if the last number of the Vec is greater than max.
/// assert_eq!(par_prime_generator(20, vec![2, 3, 5, 7, 11, 13, 17, 19, 23], 0), vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
/// ```
pub fn par_prime_generator<
    N: Roots + FromPrimitive + ToPrimitive + Copy + CheckedAdd + Send + Sync,
>(
    until: N,
    known_primes: Vec<N>,
    start_from: N,
) -> Vec<N>
where
    rayon::range::Iter<N>: IndexedParallelIterator<Item = N>,
{
    par_prime_generator_map(until, known_primes, start_from, |_| {})
}

/// Parallelely check if every number from the start to the end specified is prime and returns the primes found in a Vec.
///
/// The latest prime in the list list is used as a starting point if it is higher than the start supplied.
///
/// You can specify the behavior when a prime is found, but because they are checked in parallel they won't be mapped in order. Primes already present in the list will not be mapped.
///
/// Before checking in parallel, only one number at a time will be checked until the square root of max.
///
/// The Vec sent must be ordered. If you send an empty Vec, 2 and 3 will be added automatically.
///
/// # Examples
///
/// ```
/// use prime::generator::*;
///
/// par_prime_generator_map(15, Vec::new(), 0, |n| {println!("{}", n)}); // Prints not in order 2, 3, 5, 7, 11, 13.
/// par_prime_generator_map(15, vec![2, 3, 5], 0, |n| {println!("{}", n)}); // Prints not in order 7, 11, 13.
/// par_prime_generator_map(15, vec![2, 3, 5, 7, 11, 13, 17, 19], 0, |n| {println!("{}", n)}); // Prints nothing.
/// ```
pub fn par_prime_generator_map<
    N: Roots + FromPrimitive + ToPrimitive + Copy + CheckedAdd + Send + Sync,
    F: Fn(N) + Send + Sync,
>(
    until: N,
    mut known_primes: Vec<N>,
    start_from: N,
    found: F,
) -> Vec<N>
where
    rayon::range::Iter<N>: IndexedParallelIterator<Item = N>,
{
    get_basic_primes(&mut known_primes, until, &found);

    known_primes.reserve(
        (gauss_function(until).to_usize().unwrap()).saturating_sub(known_primes.capacity()),
    );

    let sqr = until.sqrt();

    // Because we can't modify the vector during the calculation, known_primes must be filled (single_threaded) until the square root of chunk so that we don't need to modify it.
    // However, if the vector is filled enough this step can be skipped and we can start from the last number in the vector.
    let mut start_from =
        start_from.max(known_primes.last().cloned().unwrap_or(N::zero()) + N::one() + N::one());
    if sqr > start_from {
        known_primes = prime_generator_map(sqr, known_primes, start_from, &found);
        start_from = sqr;
    }

    par_prime_generator_map_nosetup(until, known_primes, start_from, found)
}

fn oddize<N: Integer>(n: N) -> N {
    // Would use `n | 1` is the number wasn't generic
    if n.is_even() {
        n + N::one()
    } else {
        n
    }
}

fn par_prime_generator_map_nosetup<N: Roots + Copy + Send + Sync, F: Fn(N) + Send + Sync>(
    until: N,
    mut known_primes: Vec<N>,
    start_from: N,
    found: F,
) -> Vec<N>
where
    rayon::range::Iter<N>: IndexedParallelIterator<Item = N>,
{
    let start_from = oddize(start_from);
    let mut new_primes: Vec<N> = (start_from..until)
        .into_par_iter()
        .step_by(2)
        .filter(|n| is_prime(*n, known_primes.as_slice()))
        .inspect(|n| found(*n))
        .collect();
    known_primes.append(new_primes.as_mut());
    known_primes
}

// It would probably be better to have a function that asks for a custom `filter` closure,
// but it wouldn't be friendly to the borrow checker (due to `mut known_primes`, which `filter` likely uses).
fn par_prime_generator_map_nosetup_bound<N: Integer + Copy + Send + Sync, F: Fn(N) + Send + Sync>(
    until: N,
    mut known_primes: Vec<N>,
    start_from: N,
    last_i: usize,
    found: F,
) -> Vec<N>
where
    rayon::range::Iter<N>: IndexedParallelIterator<Item = N>,
{
    let start_from = oddize(start_from);
    let mut new_primes: Vec<N> = (start_from..until)
        .into_par_iter()
        .step_by(2)
        .filter(|n| is_prime_nobound(*n, &known_primes[..last_i]))
        .inspect(|n| found(*n))
        .collect();
    known_primes.append(new_primes.as_mut());
    known_primes
}

/// Parallelely check if every number from the start to the end specified is prime and returns the primes found in a Vec.
///
/// The latest prime in the list list is used as a starting point if it is higher than the start supplied.
///
/// Before every cycle, `pre_cycle` is called with the start and end of the chunk passed as input. If it returns `false`, the function will return early.
///
/// After every cycle, `post_cycle` is called with a a slice of the new primes numbers found.
///
/// Before checking in parallel, only one number at a time will be checked until the square root of the max of the first chunk.
///
/// The Vec sent must be ordered. If you send an empty Vec, 2 and 3 will be added automatically.
///
/// # Examples
///
/// ```
/// use prime::generator::*;
///
/// par_prime_generator_map_chunks(15, Vec::new(), 0, 5, |n| {println!("{}", n)}, |_, _| true); // Prints 2, 3, 5, 7, 11, 13.
/// par_prime_generator_map_chunks(15, vec![2, 3, 5], 0, 5, |n| {println!("{}", n)}, |_, _| true); // Prints 7, 11, 13.
/// par_prime_generator_map_chunks(15, vec![2, 3, 5, 7, 11, 13, 17, 19], 0, 5, |n| {println!("{}", n)}, |_, _| true); // Prints nothing.
/// ```
///
/// # Panics
///
/// The function panics if chunk_size is 0.
///
/// ```should_panic
/// use prime::generator::*;
///
/// par_prime_generator_map_chunks(15, Vec::new(), 0, 0, |n| {println!("{}", n)}, |_, _| true); // Panics!
/// ```
pub fn par_prime_generator_map_chunks<
    N: Roots + Roots + FromPrimitive + ToPrimitive + Copy + CheckedAdd + Send + Sync,
    F: FnMut(N, N) -> bool,
    G: FnMut(&[N]),
>(
    until: N,
    mut known_primes: Vec<N>,
    start_from: N,
    chunk_size: N,
    mut pre_cycle: F,
    mut post_cycle: G,
) -> Vec<N>
where
    rayon::range::Iter<N>: IndexedParallelIterator<Item = N>,
{
    assert!(!chunk_size.is_zero());

    let end = cmp::min(start_from + chunk_size, until);
    if !pre_cycle(start_from, end) {
        return known_primes;
    }

    let from = known_primes.len();
    known_primes = par_prime_generator(end, known_primes, start_from);
    post_cycle(&known_primes[from..]);

    for start in range_step(end, until, chunk_size) {
        let end = cmp::min(start + chunk_size, until);
        if !pre_cycle(start, end) {
            break;
        }

        let from = known_primes.len();
        // This is to reduce the number of square roots done.
        let last_i = last_index(&end, known_primes.as_slice());
        known_primes =
            // Could be optimized further by skipping reducing the square roots even on the preparation phase, but... who cares?
            par_prime_generator_map_nosetup_bound(end, known_primes, start, last_i, |_| {});
        post_cycle(&known_primes[from..]);
    }
    known_primes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_test() {
        assert_eq!(prime_generator(0, Vec::new(), 0), Vec::new());
        assert_eq!(
            prime_generator(20, Vec::new(), 0),
            vec![2, 3, 5, 7, 11, 13, 17, 19]
        );
        assert_eq!(
            prime_generator(20, vec![2, 3, 5, 7], 0),
            vec![2, 3, 5, 7, 11, 13, 17, 19]
        );
        assert_eq!(
            prime_generator(20, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29], 0),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
    }

    #[test]
    fn multi_generator_test() {
        assert_eq!(par_prime_generator(0, Vec::new(), 0), Vec::new());
        assert_eq!(
            par_prime_generator(20, Vec::new(), 0),
            vec![2, 3, 5, 7, 11, 13, 17, 19]
        );
        assert_eq!(
            par_prime_generator(20, vec![2, 3, 5, 7], 0),
            vec![2, 3, 5, 7, 11, 13, 17, 19]
        );
        assert_eq!(
            par_prime_generator(20, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29], 0),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
    }

    #[test]
    fn par_chunks_generator_test() {
        assert_eq!(
            par_prime_generator_map_chunks(0, Vec::new(), 0, 5, |_, _| true, |_| {}),
            Vec::new()
        );
        assert_eq!(
            par_prime_generator_map_chunks(20, Vec::new(), 0, 5, |_, _| true, |_| {}),
            vec![2, 3, 5, 7, 11, 13, 17, 19]
        );
        assert_eq!(
            par_prime_generator_map_chunks(20, vec![2, 3, 5, 7], 0, 5, |_, _| true, |_| {}),
            vec![2, 3, 5, 7, 11, 13, 17, 19]
        );
        assert_eq!(
            par_prime_generator_map_chunks(
                20,
                vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29],
                0,
                5,
                |_, _| true,
                |_| {},
            ),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
    }

    #[test]
    #[should_panic]
    fn multi_chunks_panic() {
        par_prime_generator_map_chunks(20, Vec::new(), 0, 0, |_, _| true, |_| {});
    }
}
