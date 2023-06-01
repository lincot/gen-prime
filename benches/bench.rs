use accumulator::uint::U256;
use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use gen_prime::GenPrime;
use num_bigint::BigUint;
use rand_pcg::Pcg64;

fn from_elem(c: &mut Criterion) {
    let mut rng = Pcg64::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7ac28fa16a64abf96);

    let mut group = c.benchmark_group("u8");
    group.warm_up_time(Duration::from_millis(10));
    group.measurement_time(Duration::from_millis(100));
    group.bench_with_input(BenchmarkId::from_parameter(5), &5, |b, &bits| {
        b.iter(|| u8::gen_prime(&mut rng, bits));
    });
    group.finish();

    let mut group = c.benchmark_group("u16");
    group.warm_up_time(Duration::from_millis(10));
    group.measurement_time(Duration::from_millis(100));
    group.bench_with_input(BenchmarkId::from_parameter(15), &15, |b, &bits| {
        b.iter(|| u16::gen_prime(&mut rng, bits));
    });
    group.finish();

    let mut group = c.benchmark_group("u32");
    group.warm_up_time(Duration::from_millis(10));
    group.measurement_time(Duration::from_millis(200));
    for bits in 30..=31 {
        group.bench_with_input(BenchmarkId::from_parameter(bits), &bits, |b, &bits| {
            b.iter(|| u32::gen_prime(&mut rng, bits));
        });
    }
    group.finish();

    let mut group = c.benchmark_group("u64");
    group.warm_up_time(Duration::from_millis(10));
    group.measurement_time(Duration::from_millis(200));
    for bits in 50..=51 {
        group.bench_with_input(BenchmarkId::from_parameter(bits), &bits, |b, &bits| {
            b.iter(|| u64::gen_prime(&mut rng, bits));
        });
    }
    group.finish();

    let mut group = c.benchmark_group("u256");
    group.warm_up_time(Duration::from_millis(10));
    group.measurement_time(Duration::from_millis(500));
    for bits in 250..=251 {
        group.bench_with_input(BenchmarkId::from_parameter(bits), &bits, |b, &bits| {
            b.iter(|| U256::gen_prime(&mut rng, bits));
        });
    }
    group.finish();

    let mut group = c.benchmark_group("BigUint low");
    group.warm_up_time(Duration::from_millis(10));
    group.measurement_time(Duration::from_millis(1000));
    group.bench_with_input(BenchmarkId::from_parameter(250), &250, |b, &bits| {
        b.iter(|| BigUint::gen_prime(&mut rng, bits));
    });
    group.finish();

    let mut group = c.benchmark_group("BigUint");
    group.warm_up_time(Duration::from_millis(10));
    group.measurement_time(Duration::from_millis(2000));
    for bits in 500..=501 {
        group.bench_with_input(BenchmarkId::from_parameter(bits), &bits, |b, &bits| {
            b.iter(|| BigUint::gen_prime(&mut rng, bits));
        });
    }
    group.finish();
}

criterion_group!(benches, from_elem);
criterion_main!(benches);
