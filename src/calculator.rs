use num::{integer::Roots, Integer};
use rayon::prelude::*;

#[inline]
pub(crate) fn last_index<N: Roots>(n: &N, known_primes: &[N]) -> usize {
    match known_primes.binary_search(&n.sqrt()) {
        Ok(i) => i + 1,
        Err(i) => i,
    }
}

/// Calculates if the number n is prime by iterating through the known primes passed in.
///
/// Iteration breaks (returning true) after reaching the square root of n. It's required for the list to be ordered.
#[inline]
pub fn is_prime<N: Roots>(n: N, known_primes: &[N]) -> bool {
    if n.is_zero() || n.is_one() {
        return false;
    };

    let last_i = last_index(&n, known_primes);
    is_prime_nobound(n, &known_primes[..last_i])
}

/// Calculates if the number n is prime by iterating through the known primes passed in.
///
/// All numbers in the list will be checked for the module operation.
///
/// Use it, for instance, if you want to force one specific sqrt for a chunk of numbers.
#[inline]
pub fn is_prime_nobound<N: Integer>(n: N, list: &[N]) -> bool {
    !list.iter().any(|i| (n.is_multiple_of(i)))
}

/// Calculates if the number n is prime by iterating in parallel through only the known primes passed in.
///
/// Iteration only goes up to the square root of n. It's required for the list to be ordered.
#[inline]
pub fn par_is_prime<N>(n: N, known_primes: &[N]) -> bool
where
    N: Roots + Sync, /* to implement IntoParIter */
{
    if n.is_zero() || n.is_one() {
        return false;
    };

    let last_i = last_index(&n, known_primes);
    par_is_prime_nobound(n, &known_primes[..last_i])
}

/// Calculates in parallel if the number n is prime by iterating through the known primes passed in.
///
/// All numbers in the list will be checked for the module operation.
///
/// Use it, for instance, if you want to force one specific sqrt for a chunk of numbers.
#[inline]
pub fn par_is_prime_nobound<N>(n: N, list: &[N]) -> bool
where
    N: Roots + Sync, /* to implement IntoParIter */
{
    !list.par_iter().any(|i| (n.is_multiple_of(i)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_test() {
        assert!(!is_prime(0, &[2, 3, 5, 7]));
        assert!(!is_prime(1, &[2, 3, 5, 7]));
        assert!(is_prime(2, &[2, 3, 5, 7]));
        assert!(is_prime(3, &[2, 3, 5, 7]));
        assert!(is_prime(5, &[2, 3, 5, 7]));
        assert!(!is_prime(6, &[2, 3, 5, 7]));
        assert!(is_prime(7, &[2, 3, 5, 7]));
        assert!(!is_prime(9, &[2, 3, 5, 7]));
    }

    #[test]
    fn par_prime_test() {
        assert!(!par_is_prime(0, &[2, 3]));
        assert!(!par_is_prime(1, &[2, 3]));
        assert!(par_is_prime(2, &[2, 3]));
        assert!(par_is_prime(3, &[2, 3]));
        assert!(par_is_prime(5, &[2, 3]));
        assert!(par_is_prime(2, &[2, 3, 5, 7]));
        assert!(par_is_prime(3, &[2, 3, 5, 7]));
        assert!(par_is_prime(5, &[2, 3, 5, 7]));
        assert!(!par_is_prime(6, &[2, 3, 5, 7]));
        assert!(par_is_prime(7, &[2, 3, 5, 7]));
        assert!(!par_is_prime(9, &[2, 3, 5, 7]));
    }
}
