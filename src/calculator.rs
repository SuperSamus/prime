use integer_sqrt::IntegerSquareRoot;
use rayon::prelude::*;

/// Calculates if the number n is prime by iterating through the known primes passed in.
///
/// Iteration breaks (returning true) after reaching the square root of n, thus it's required for the list to be ordered.
#[inline]
pub fn is_prime(n: u64, known_primes: &[u64]) -> bool {
    if n == 0 || n == 1 {
        return false;
    };
    let sqr = n.integer_sqrt();

    let last_index = match known_primes.binary_search(&sqr) {
        Ok(i) => i + 1,
        Err(i) => i,
    };
    for i in known_primes[0..last_index].iter() {
        if n % *i == 0 {
            return false;
        }
    }
    true
}

/// Calculates if the number n is prime by iterating in parallel through only the known primes passed in.
///
/// Iteration only goes up to the square root of n, thus it's required for the list to be ordered.
#[inline]
pub fn par_is_prime(n: u64, known_primes: &[u64]) -> bool {
    if n == 0 || n == 1 {
        return false;
    };
    let sqr = n.integer_sqrt();
    let last_index = match known_primes.binary_search(&sqr) {
        Ok(i) => i + 1,
        Err(i) => i,
    };
    !known_primes[0..last_index].par_iter().any(|i| n % i == 0)
}

/// Calculates if the number n is prime by iterating through the known primes passed in.
///
/// If the list ends and the last number is less than the square root of n, the iteration will continue by adding 2 to the last number.
///
/// Iteration breaks (returning true) after reaching the square root of n, thus it's required for the list to be ordered.
pub fn ignorant_is_prime(n: u64, known_primes: &[u64]) -> bool {
    if n == 2 {
        return true;
    };
    if n % 2 == 0 || n == 1 {
        return false;
    };
    let sqr = n.integer_sqrt();
    !IgnorantPrimeIterator::new(sqr, known_primes).any(|i| n % i == 0)
}

/// An iterator the cycles through the primes given, than adds 2 until max is reached.
struct IgnorantPrimeIterator<'a> {
    max: u64,
    known_primes: &'a [u64],
    current_index: usize,
    current_num: u64,
}

impl<'a> IgnorantPrimeIterator<'a> {
    fn new(max: u64, known_primes: &'a [u64]) -> Self {
        IgnorantPrimeIterator {
            max,
            known_primes,
            current_index: 0,
            current_num: 1,
        }
    }
}

impl<'a> Iterator for IgnorantPrimeIterator<'a> {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        // Iterate through the list, after the list is over add 2.
        match self.known_primes.get(self.current_index) {
            Some(n) => {
                self.current_num = *n;
                self.current_index += 1;
            }
            None => {
                // If the latest number is even (2), then 1 must be added to make current_num odd.
                if self.current_num % 2 != 0 {
                    self.current_num += 2;
                } else {
                    self.current_num += 1;
                }
            }
        }
        if self.current_num <= self.max {
            Some(self.current_num)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_test() {
        assert_eq!(is_prime(0, &[2, 3, 5, 7]), false);
        assert_eq!(is_prime(1, &[2, 3, 5, 7]), false);
        assert_eq!(is_prime(2, &[2, 3, 5, 7]), true);
        assert_eq!(is_prime(3, &[2, 3, 5, 7]), true);
        assert_eq!(is_prime(5, &[2, 3, 5, 7]), true);
        assert_eq!(is_prime(6, &[2, 3, 5, 7]), false);
        assert_eq!(is_prime(7, &[2, 3, 5, 7]), true);
        assert_eq!(is_prime(9, &[2, 3, 5, 7]), false);
    }

    #[test]
    fn par_prime_test() {
        assert_eq!(par_is_prime(0, &[2, 3]), false);
        assert_eq!(par_is_prime(1, &[2, 3]), false);
        assert_eq!(par_is_prime(2, &[2, 3]), true);
        assert_eq!(par_is_prime(3, &[2, 3]), true);
        assert_eq!(par_is_prime(5, &[2, 3]), true);
        assert_eq!(par_is_prime(2, &[2, 3, 5, 7]), true);
        assert_eq!(par_is_prime(3, &[2, 3, 5, 7]), true);
        assert_eq!(par_is_prime(5, &[2, 3, 5, 7]), true);
        assert_eq!(par_is_prime(6, &[2, 3, 5, 7]), false);
        assert_eq!(par_is_prime(7, &[2, 3, 5, 7]), true);
        assert_eq!(par_is_prime(9, &[2, 3, 5, 7]), false);
    }

    #[test]
    fn ignorant_empty_prime_test() {
        assert_eq!(ignorant_is_prime(0, &[]), false);
        assert_eq!(ignorant_is_prime(1, &[]), false);
        assert_eq!(ignorant_is_prime(2, &[]), true);
        assert_eq!(ignorant_is_prime(3, &[]), true);
        assert_eq!(ignorant_is_prime(5, &[]), true);
        assert_eq!(ignorant_is_prime(6, &[]), false);
        assert_eq!(ignorant_is_prime(7, &[]), true);
        assert_eq!(ignorant_is_prime(9, &[]), false);
    }

    #[test]
    fn ignorant_even_prime_test() {
        assert_eq!(ignorant_is_prime(0, &[2]), false);
        assert_eq!(ignorant_is_prime(1, &[2]), false);
        assert_eq!(ignorant_is_prime(2, &[2]), true);
        assert_eq!(ignorant_is_prime(3, &[2]), true);
        assert_eq!(ignorant_is_prime(5, &[2]), true);
        assert_eq!(ignorant_is_prime(6, &[2]), false);
        assert_eq!(ignorant_is_prime(7, &[2]), true);
        assert_eq!(ignorant_is_prime(9, &[2]), false);
    }

    #[test]
    fn ignorant_prime_test() {
        assert_eq!(ignorant_is_prime(0, &[2, 3, 5, 7]), false);
        assert_eq!(ignorant_is_prime(1, &[2, 3, 5, 7]), false);
        assert_eq!(ignorant_is_prime(2, &[2, 3, 5, 7]), true);
        assert_eq!(ignorant_is_prime(3, &[2, 3, 5, 7]), true);
        assert_eq!(ignorant_is_prime(5, &[2, 3, 5, 7]), true);
        assert_eq!(ignorant_is_prime(6, &[2, 3, 5, 7]), false);
        assert_eq!(ignorant_is_prime(7, &[2, 3, 5, 7]), true);
        assert_eq!(ignorant_is_prime(9, &[2, 3, 5, 7]), false);
        assert_eq!(ignorant_is_prime(11, &[2, 3, 5, 7]), true);
        assert_eq!(ignorant_is_prime(13, &[2, 3, 5, 7]), true);
        assert_eq!(ignorant_is_prime(15, &[2, 3, 5, 7]), false);
    }
}
