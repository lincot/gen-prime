use accumulator::uint::U256;
use gen_prime::{u256_to_biguint, GenPrime};
use num_bigint::BigUint;
use rand::rngs::OsRng;
use std::env;

fn main() {
    const EMSG: &str = "please pass the number of bits";
    let bits = env::args().nth(1).expect(EMSG).parse().expect(EMSG);
    assert!(bits > 1);

    let mut rng = OsRng;
    if bits <= 8 {
        println!("{}", u8::gen_prime(&mut rng, bits));
    } else if bits <= 16 {
        println!("{}", u16::gen_prime(&mut rng, bits));
    } else if bits <= 32 {
        println!("{}", u32::gen_prime(&mut rng, bits));
    } else if bits <= 64 {
        println!("{}", u64::gen_prime(&mut rng, bits));
    } else if bits <= 256 {
        println!("{}", u256_to_biguint(U256::gen_prime(&mut rng, bits)));
    } else {
        println!("{}", BigUint::gen_prime(&mut rng, bits));
    }
}
