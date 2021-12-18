extern crate num;

use num::{bigint::BigUint, One, Zero};

fn factorial(value: u32) -> BigUint {
    (2..=value).fold(BigUint::one(), |res, n| res * n)
}

fn smaller_values(val: u8, data: &[u8]) -> u8 {
    let mut total = 0;
    for &num in data {
        if num < val {
            total += 1
        }
    }
    total
}

pub fn compute(data: &[u8]) -> String {
    // dont need to check last value as last value will have 0 smaller values on the right (0*0!)
    let mut result: BigUint = Zero::zero();
    for i in 0..data.len() - 1 {
        result +=
            smaller_values(data[i], &data[i + 1..]) * factorial(data.len() as u32 - 1 - i as u32);
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::bigint::ToBigUint;

    #[test]
    fn factorial_test() {
        assert_eq!(1.to_biguint().unwrap(), factorial(0));
        assert_eq!(1.to_biguint().unwrap(), factorial(1));
        assert_eq!(2.to_biguint().unwrap(), factorial(2));
        assert_eq!(40320.to_biguint().unwrap(), factorial(8));
        assert_eq!(479001600.to_biguint().unwrap(), factorial(12));
    }

    #[test]
    fn smaller_values_test() {
        assert_eq!(2, smaller_values(3, &[1, 2, 3]));
        assert_eq!(0, smaller_values(1, &[1, 2, 3]));
        assert_eq!(1, smaller_values(2, &[1, 2, 3]));
    }

    #[test]
    fn compute_test() {
        assert_eq!("5", compute(&[3, 2, 1]));
        assert_eq!("0", compute(&[1, 2, 3]));
        assert_eq!("3", compute(&[2, 3, 1]));
    }
}
