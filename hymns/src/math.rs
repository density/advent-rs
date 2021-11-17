use num_traits::PrimInt;
use std::iter::Product;

// See https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
pub fn extended_euclidean<T: PrimInt>(a: T, b: T) -> (T, T, T) {
    if a.is_zero() {
        (b, T::zero(), T::one())
    } else {
        let (gcd, x, y) = extended_euclidean(b % a, a);
        (gcd, y - (b / a) * x, x)
    }
}

pub fn modular_inverse<T: PrimInt>(x: T, n: T) -> Option<T> {
    let (gcd, x, _) = extended_euclidean(x, n);
    if gcd.is_one() {
        Some((x % n + n) % n)
    } else {
        None
    }
}

pub fn chinese_remainder<T: PrimInt + Product>(rems: &[T], mods: &[T]) -> Option<T> {
    let prod: T = mods.iter().cloned().product();

    let sum = rems
        .iter()
        .zip(mods)
        .try_fold(T::zero(), |acc, (&rem, &modulus)| {
            let p = prod / modulus;
            let mod_inv = modular_inverse(p, modulus)?;
            Some(acc + rem * mod_inv * p)
        })?;

    Some(sum % prod)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(extended_euclidean(10, 0), (10, 1, 0));
        assert_eq!(extended_euclidean(10, 3), (1, 1, -3));
        assert_eq!(extended_euclidean(10, -3), (1, 1, 3));
        assert_eq!(extended_euclidean(-25, 15), (5, 1, 2));
    }

    #[test]
    fn test_crt() {
        assert_eq!(chinese_remainder(&[0, 3, 4], &[3, 4, 5]), Some(39));
        assert_eq!(chinese_remainder(&[0, 3], &[3, 6]), None);
    }

    #[test]
    fn test_mod_inv() {
        assert_eq!(modular_inverse(10, 3), Some(1));
        assert_eq!(modular_inverse(4, 2), None);
    }
}
