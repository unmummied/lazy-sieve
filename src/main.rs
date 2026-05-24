use itertools::iterate;
use std::collections::HashMap;

fn main() {
    let n = 1_000;
    if let Some((i, p)) = primes()
        .enumerate()
        .take_while(|&(_, p)| p < 1000)
        .last()
    {
        println!("the largest prime less than {n} is {p} the {}-th prime!", i + 1);
    }
}

fn primes() -> impl Iterator<Item = u32> {
    let mut map = HashMap::from([(4, 2)]);
    iterate((2u32, true), move |(pred, _)| {
        let n = pred + 1;
        let is_prime = match map.remove(&n) {
            None => {
                if let Some(square) = n.checked_mul(n) {
                    map.insert(square, n);
                }
                true
            }
            Some(p) => {
                let mut skipped = n + p;
                while map.contains_key(&skipped) {
                    skipped += p;
                }
                map.insert(skipped, p);
                false
            }
        };
        (n, is_prime)
    })
    .filter_map(|(n, is_prime)| is_prime.then_some(n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes() {
        fn is_prime(n: u32) -> bool {
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

        let naive = (2..1_000_000).filter(|&n| is_prime(n));
        let lazy = primes();
        assert!(naive.zip(lazy).into_iter().all(|(n, l)| n == l));
    }
}
