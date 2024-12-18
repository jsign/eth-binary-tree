use criterion::{black_box, criterion_group, criterion_main, Criterion};
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

criterion_group!(benches, blake3_benchmark, keccak_benchmark);
criterion_main!(benches);
