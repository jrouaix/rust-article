use itertools::Itertools;
use rayon::prelude::*;
use std::sync::*;

// idée d'exemple de Cécile T. : parser des logs et en tirer des stats

fn main() {
    const N: u128 = 10000;
    println!("{}", get_primes_before(N).iter().join(", "));
}

fn get_primes_before(max: u128) -> Vec<u128> {
    let previous = RwLock::new(vec![]);
    (0..max)
        .into_par_iter()
        .filter(|i| {
            let prime = {
                let reader = previous.read().unwrap();
                is_prime(i, &*reader)
            };
            if prime {
                let mut writer = previous.write().unwrap();
                writer.push(*i);
            }
            prime
        })
        .collect()
}

fn is_prime(number: &u128, previous_ones: &Vec<u128>) -> bool {
    if *number <= 1 {
        return false;
    }
    let mut last_previous = 2;
    for i in previous_ones.iter() {
        if number % i == 0 && number != i {
            return false;
        }
        last_previous = *i;
    }
    for i in last_previous + 1..*number {
        if number % i == 0 && *number != i {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*; // pour ajouter tout ce est au dessus au scope
    use std::time::Instant;

    #[test]
    fn performances() {
        let now = Instant::now();
        let _test = get_primes_before(100_000);
        dbg!(_test.len(), now.elapsed());
    }

    #[test]
    fn is_prime_tests() {
        let test_them_all: Vec<_> = (2u128..100).collect();
        assert_eq!(is_prime(&1, &test_them_all), false);
        assert_eq!(is_prime(&4, &test_them_all), false);
        assert_eq!(is_prime(&6, &test_them_all), false);
        assert_eq!(is_prime(&2, &test_them_all), true);
        assert_eq!(is_prime(&3, &test_them_all), true);
        assert_eq!(is_prime(&5, &test_them_all), true);
        assert_eq!(is_prime(&7, &test_them_all), true);
        assert_eq!(is_prime(&11, &test_them_all), true);
        assert_eq!(is_prime(&13, &test_them_all), true);
        assert_eq!(is_prime(&17, &test_them_all), true);
        assert_eq!(is_prime(&19, &test_them_all), true);
        assert_eq!(is_prime(&23, &test_them_all), true);
        assert_eq!(is_prime(&29, &test_them_all), true);
        assert_eq!(is_prime(&31, &test_them_all), true);
        assert_eq!(is_prime(&37, &test_them_all), true);
        assert_eq!(is_prime(&41, &test_them_all), true);
        assert_eq!(is_prime(&42, &test_them_all), false);
        assert_eq!(is_prime(&43, &test_them_all), true);
        assert_eq!(is_prime(&47, &test_them_all), true);
        assert_eq!(is_prime(&53, &test_them_all), true);
        assert_eq!(is_prime(&59, &test_them_all), true);
        assert_eq!(is_prime(&61, &test_them_all), true);
        assert_eq!(is_prime(&63, &test_them_all), false);
        assert_eq!(is_prime(&67, &test_them_all), true);
        assert_eq!(is_prime(&71, &test_them_all), true);
        assert_eq!(is_prime(&73, &test_them_all), true);
        assert_eq!(is_prime(&79, &test_them_all), true);
        assert_eq!(is_prime(&83, &test_them_all), true);
        assert_eq!(is_prime(&89, &test_them_all), true);
        assert_eq!(is_prime(&97, &test_them_all), true);
    }
}
