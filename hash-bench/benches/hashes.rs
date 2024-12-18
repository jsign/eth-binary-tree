use criterion::{black_box, criterion_group, criterion_main, Criterion};
use p3_baby_bear::{BabyBearParameters, Poseidon2BabyBear};
use p3_koala_bear::{KoalaBearParameters, Poseidon2KoalaBear};
use p3_mersenne_31::{Mersenne31, Poseidon2Mersenne31};
use p3_monty_31::MontyField31;
use p3_symmetric::Permutation;
use tiny_keccak::Hasher;

pub fn blake3_benchmark(c: &mut Criterion) {
    let input = [0x41; 64];
    for size in [32, 64] {
        let input = &input[..size];
        c.bench_function(&format!("blake3_{}-bytes", size), |b| {
            b.iter(|| blake3::hash(black_box(input)))
        });
    }
}

pub fn keccak_benchmark(c: &mut Criterion) {
    let input = [0x41; 64];
    for size in [32, 64] {
        c.bench_function(&format!("keccak_{}-bytes", size), |b| {
            let input = &input[..size];
            let mut output = [0x00; 32];
            b.iter(|| {
                let mut h = tiny_keccak::Keccak::v256();
                h.update(black_box(input));
                h.finalize(&mut output);
            })
        });
    }
}

pub fn poseidon2_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    c.bench_function("poseidon2_KoalaBear_16", |b| {
        let perm = Poseidon2KoalaBear::<16>::new_from_rng_128(&mut rng);
        let mut input: [MontyField31<KoalaBearParameters>; 16] = rand::random();
        b.iter(|| perm.permute_mut(&mut input))
    });
    c.bench_function("poseidon2_BabyBear_16", |b| {
        let perm = Poseidon2BabyBear::<16>::new_from_rng_128(&mut rng);
        let mut input: [MontyField31<BabyBearParameters>; 16] = rand::random();
        b.iter(|| perm.permute_mut(&mut input))
    });
    c.bench_function("poseidon2_Mersenne31_16", |b| {
        let perm = Poseidon2Mersenne31::<16>::new_from_rng_128(&mut rng);
        let mut input: [Mersenne31; 16] = rand::random();
        b.iter(|| perm.permute_mut(&mut input))
    });
}

criterion_group!(
    benches,
    blake3_benchmark,
    keccak_benchmark,
    poseidon2_benchmark
);
criterion_main!(benches);
