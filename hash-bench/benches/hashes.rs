use criterion::{criterion_group, criterion_main, BatchSize::SmallInput, Criterion};
use p3_baby_bear::{BabyBear, Poseidon2BabyBear};
use p3_field::Field;
use p3_koala_bear::{KoalaBear, Poseidon2KoalaBear};
use p3_mersenne_31::{Mersenne31, Poseidon2Mersenne31};
use p3_symmetric::Permutation;
use rand::{distributions::Standard, prelude::Distribution, RngCore};
use tiny_keccak::Hasher;

pub fn blake3_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    for size in [32, 64] {
        c.bench_function(&format!("blake3_{}-bytes", size), |b| {
            b.iter_batched(
                || {
                    let mut input = vec![0u8; size];
                    rng.fill_bytes(&mut input);
                    input
                },
                |input| blake3::hash(&input),
                SmallInput,
            )
        });
    }
}

pub fn keccak_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    for size in [32, 64] {
        c.bench_function(&format!("keccak_{}-bytes", size), |b| {
            b.iter_batched(
                || {
                    let mut input = vec![0u8; size];
                    rng.fill_bytes(&mut input);
                    input
                },
                |input| {
                    let mut h = tiny_keccak::Keccak::v256();
                    h.update(&input);
                    h.finalize(&mut [0u8; 32])
                },
                SmallInput,
            )
        });
    }
}

pub fn poseidon2_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    poseidon2_bench::<KoalaBear, _>(
        c,
        Poseidon2KoalaBear::<16>::new_from_rng_128(&mut rng),
        "poseidon2_KoalaBear_16",
    );
    poseidon2_bench::<BabyBear, _>(
        c,
        Poseidon2BabyBear::<16>::new_from_rng_128(&mut rng),
        "poseidon2_BabyBear_16",
    );
    poseidon2_bench::<Mersenne31, _>(
        c,
        Poseidon2Mersenne31::<16>::new_from_rng_128(&mut rng),
        "poseidon2_Mersenne31_16",
    );
}

fn poseidon2_bench<F, Perm>(c: &mut Criterion, perm: Perm, name: &str)
where
    F: Field,
    Perm: Permutation<[F::Packing; 16]>,
    Standard: Distribution<[F::Packing; 16]>,
{
    c.bench_function(name, |b| {
        b.iter_batched_ref(
            || {
                let input: [F::Packing; 16] = rand::random();
                input
            },
            |input| perm.permute_mut(input),
            SmallInput,
        );
    });
}

criterion_group!(
    benches,
    blake3_benchmark,
    keccak_benchmark,
    poseidon2_benchmark
);
criterion_main!(benches);
