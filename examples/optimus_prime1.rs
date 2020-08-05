use itertools::Itertools;

fn main() {
    const N: u128 = 10000;
    println!("{}", get_primes_before(N).iter().join(", "));
}

fn get_primes_before(max: u128) -> Vec<u128> {
    (0..max).filter(is_prime).collect()
}

fn is_prime(number: &u128) -> bool {
    if *number <= 1 {
        return false;
    }
    for i in 2..*number {
        if number % i == 0 {
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
        let _test = get_primes_before(50000);
        dbg!(now.elapsed());
    }

    #[test]
    fn is_prime_tests() {
        assert_eq!(is_prime(&1), false);
        assert_eq!(is_prime(&4), false);
        assert_eq!(is_prime(&6), false);
        assert_eq!(is_prime(&2), true);
        assert_eq!(is_prime(&3), true);
        assert_eq!(is_prime(&5), true);
        assert_eq!(is_prime(&7), true);
        assert_eq!(is_prime(&11), true);
        assert_eq!(is_prime(&13), true);
        assert_eq!(is_prime(&17), true);
        assert_eq!(is_prime(&19), true);
        assert_eq!(is_prime(&23), true);
        assert_eq!(is_prime(&29), true);
        assert_eq!(is_prime(&31), true);
        assert_eq!(is_prime(&37), true);
        assert_eq!(is_prime(&41), true);
        assert_eq!(is_prime(&42), false);
        assert_eq!(is_prime(&43), true);
        assert_eq!(is_prime(&47), true);
        assert_eq!(is_prime(&53), true);
        assert_eq!(is_prime(&59), true);
        assert_eq!(is_prime(&61), true);
        assert_eq!(is_prime(&63), false);
        assert_eq!(is_prime(&67), true);
        assert_eq!(is_prime(&71), true);
        assert_eq!(is_prime(&73), true);
        assert_eq!(is_prime(&79), true);
        assert_eq!(is_prime(&83), true);
        assert_eq!(is_prime(&89), true);
        assert_eq!(is_prime(&97), true);
    }
}
