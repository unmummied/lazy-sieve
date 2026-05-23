use std::{collections::HashMap, iter::successors};

fn main() {
    for (i, p) in primes().enumerate().take(100) {
        println!("{:>3}th prime is {p:>3}!", i + 1);
    }
}

fn primes() -> impl Iterator<Item = u64> {
    let mut map = HashMap::from([(4, 2)]);
    successors(Some((2u64, true)), move |&(pred, _)| {
        let n = pred + 1;
        match map.remove(&n) {
            None => {
                let square = n.checked_mul(n)?;
                map.insert(square, n);
                Some((n, true))
            }
            Some(p) => {
                let mut next = n + p;
                while map.contains_key(&next) {
                    next += p;
                }
                map.insert(next, p);
                Some((n, false))
            }
        }
    })
    .filter_map(|(n, is_prime)| is_prime.then_some(n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes() {
        fn is_prime(n: u64) -> bool {
            if n < 3 {
                return n == 2;
            }
            if n.is_multiple_of(2) {
                return false;
            }
            let mut d = 3;
            while d <= n.isqrt() {
                if n.is_multiple_of(d) {
                    return false;
                }
                d += 2;
            }
            true
        }

        let naive = (2..10_000_000).filter(|&n| is_prime(n));
        let lazy = primes();
        assert!(naive.zip(lazy).into_iter().all(|(n, l)| n == l));
    }
}
