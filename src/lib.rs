#![no_std]

use accumulator::{hash::primality::is_prob_prime, uint::U256};
use core::mem::transmute;
use glass_pumpkin::prime;
use num_bigint::{BigUint, RandBigInt};
use num_prime::nt_funcs::is_prime64;
use rand::seq::SliceRandom;
use rand::Rng;

mod primes;

trait GenSignificantBits {
    /// generate `Self` with `bits` significant bits
    fn gen_significant_bits(rng: &mut impl Rng, bits: u32) -> Self;
}

macro_rules! gen_significant_bits_impl {
    ($($t:ty)*) => ($(
        impl GenSignificantBits for $t {
            fn gen_significant_bits(rng: &mut impl Rng, bits: u32) -> Self {
                let mut n: Self = rng.gen();
                n &= 1 << (Self::BITS - 1);
                n >>= Self::BITS - bits;
                n
            }
        }
    )*)
}

gen_significant_bits_impl! { u32 u64 }

impl GenSignificantBits for U256 {
    fn gen_significant_bits(rng: &mut impl Rng, bits: u32) -> U256 {
        let mut digits = [0; 4];
        let len = ((bits + 63) / 64) as _;
        let rem = bits % 64;
        rng.fill(&mut digits[..len]);
        digits[len - 1] &= 1 << 63;
        digits[len - 1] >>= 64 - rem;
        U256::from(digits)
    }
}

impl GenSignificantBits for BigUint {
    fn gen_significant_bits(rng: &mut impl Rng, bits: u32) -> Self {
        let mut n = rng.gen_biguint((bits - 1) as _);
        n.set_bit((bits - 1) as _, true);
        n
    }
}

pub trait FindPrimeFrom {
    /// find the lowest prime number, starting from `self`
    fn find_prime_from(self) -> Self;
}

macro_rules! find_prime_from_impl {
    ($($t:ty)*) => ($(
        impl FindPrimeFrom for $t {
            fn find_prime_from(mut self) -> Self {
                if self <= 2 {
                    return 2;
                }
                if self == 3 {
                    return 3;
                }

                let r = self % 6;
                let (next_r, mut step) = if r <= 1 { (1, 4) } else { (5, 2) };
                self += next_r - r;

                loop {
                    if is_prime64(self as _) {
                        return self;
                    }

                    self += step;
                    step ^= 6;
                }
            }
        }
    )*)
}

find_prime_from_impl! { u32 u64 }

impl FindPrimeFrom for U256 {
    fn find_prime_from(mut self) -> Self {
        if self <= 2.into() {
            return 2.into();
        }
        if self == 3u64 {
            return 3.into();
        }
        let r = self % Self::from(6);
        let (r, _): (u8, [u8; 39]) = unsafe { transmute(r) };
        let (next_r, mut step) = if r <= 1 { (1, 4) } else { (5, 2) };
        self += ((next_r - r) as u64).into();

        loop {
            if is_prob_prime(&self) {
                return self;
            }

            self += step.into();
            step ^= 6;
        }
    }
}

impl FindPrimeFrom for BigUint {
    fn find_prime_from(mut self) -> Self {
        if self <= 2u8.into() {
            return 2u8.into();
        }
        if self == 3u8.into() {
            return 3u8.into();
        }
        let r: u8 = (&self % 6u8).try_into().unwrap();
        let (next_r, mut step) = if r <= 1 { (1, 4) } else { (5, 2u8) };
        self += next_r - r;

        loop {
            if prime::check(&self) {
                return self;
            }

            self += step;
            step ^= 6;
        }
    }
}

pub trait GenPrime {
    /// generate a prime number with `bits` significant bits
    fn gen_prime(rng: &mut impl Rng, bits: u32) -> Self;
}

macro_rules! gen_prime_impl {
    ($($t:ty)*) => ($(
        impl GenPrime for $t {
            fn gen_prime(rng: &mut impl Rng, bits: u32) -> Self {
                assert!((2..=Self::BITS).contains(&bits));
                loop {
                    let prime = Self::gen_significant_bits(rng, bits).find_prime_from();
                    if prime.leading_zeros() == Self::BITS - bits {
                        return prime;
                    }
                }
            }
        }
    )*)
}

gen_prime_impl! { u32 u64 }

impl GenPrime for u8 {
    fn gen_prime(rng: &mut impl Rng, bits: u32) -> Self {
        match bits {
            2 => *primes::PRIMES_2.choose(rng).unwrap(),
            3 => *primes::PRIMES_3.choose(rng).unwrap(),
            4 => *primes::PRIMES_4.choose(rng).unwrap(),
            5 => *primes::PRIMES_5.choose(rng).unwrap(),
            6 => *primes::PRIMES_6.choose(rng).unwrap(),
            7 => *primes::PRIMES_7.choose(rng).unwrap(),
            8 => *primes::PRIMES_8.choose(rng).unwrap(),
            _ => panic!(),
        }
    }
}

impl GenPrime for u16 {
    fn gen_prime(rng: &mut impl Rng, bits: u32) -> Self {
        match bits {
            bits if bits <= 8 => u8::gen_prime(rng, bits) as _,
            9 => *primes::PRIMES_9.choose(rng).unwrap(),
            10 => *primes::PRIMES_10.choose(rng).unwrap(),
            11 => *primes::PRIMES_11.choose(rng).unwrap(),
            12 => *primes::PRIMES_12.choose(rng).unwrap(),
            13 => *primes::PRIMES_13.choose(rng).unwrap(),
            14 => *primes::PRIMES_14.choose(rng).unwrap(),
            15 => *primes::PRIMES_15.choose(rng).unwrap(),
            16 => *primes::PRIMES_16.choose(rng).unwrap(),
            _ => panic!(),
        }
    }
}

impl GenPrime for U256 {
    fn gen_prime(rng: &mut impl Rng, bits: u32) -> Self {
        assert!((2..=256).contains(&bits));
        let lbound = U256::from(1) << (bits - 1);
        let ubound = if bits == 256 {
            [u64::MAX; 4].into()
        } else {
            U256::from(1) << bits
        };
        loop {
            let prime = Self::gen_significant_bits(rng, bits).find_prime_from();
            if (&lbound..&ubound).contains(&&prime) {
                return prime;
            }
        }
    }
}

impl GenPrime for BigUint {
    fn gen_prime(rng: &mut impl Rng, bits: u32) -> Self {
        assert!(bits >= 2);
        loop {
            let prime = Self::gen_significant_bits(rng, bits).find_prime_from();
            if prime.bits() == bits as u64 {
                return prime;
            }
        }
    }
}

pub fn u256_to_biguint(n: U256) -> BigUint {
    let (digits, _): ([u32; 8], i64) = unsafe { transmute(n) };
    BigUint::from_slice(&digits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::fmt::Debug;
    use rand_pcg::Pcg64;

    #[test]
    fn test_find_prime_from() {
        fn test<T: FindPrimeFrom + Eq + Debug + TryFrom<u64>>(a: u64, b: u64) {
            assert_eq!(
                TryInto::<T>::try_into(a)
                    .unwrap_or_else(|_| panic!())
                    .find_prime_from(),
                b.try_into().unwrap_or_else(|_| panic!())
            );
        }

        fn test_group<T: FindPrimeFrom + Eq + Debug + TryFrom<u64>>() {
            test::<T>(0, 2);
            test::<T>(1, 2);
            test::<T>(2, 2);
            test::<T>(3, 3);
            test::<T>(4, 5);
            test::<T>(5, 5);
            test::<T>(6, 7);
            test::<T>(98, 101);
            test::<T>(360, 367);
            test::<T>(7855, 7867);
            test::<T>(7855, 7867);
            test::<T>(7919, 7919);
        }

        test_group::<u32>();
        test_group::<u64>();
        test::<u32>(834219510, 834219521);
        test_group::<U256>();
        test::<U256>(2609232758985927650, 2609232758985927689);
        test_group::<BigUint>();
        test::<BigUint>(2609232758985927650, 2609232758985927689);
    }

    #[test]
    fn test_gen_prime() {
        let mut rng = Pcg64::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7ac28fa16a64abf96);
        assert_eq!(u8::gen_prime(&mut rng, 5).leading_zeros(), u8::BITS - 5);
        assert_eq!(u16::gen_prime(&mut rng, 10).leading_zeros(), u16::BITS - 10);
        assert_eq!(u32::gen_prime(&mut rng, 30).leading_zeros(), u32::BITS - 30);
        assert_eq!(u64::gen_prime(&mut rng, 51).leading_zeros(), u64::BITS - 51);
        assert_eq!(u256_to_biguint(U256::gen_prime(&mut rng, 200)).bits(), 200);
        assert_eq!(BigUint::gen_prime(&mut rng, 500).bits(), 500);
    }
}
