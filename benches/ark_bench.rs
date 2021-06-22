#[macro_use]
extern crate criterion;

use ark_ec::AffineCurve;
use ark_ec::ProjectiveCurve;
use ark_std::ops::MulAssign;
use ark_std::rand::{RngCore, SeedableRng};
use ark_std::UniformRand;
use criterion::Criterion;
use rand_chacha::ChaCha20Rng;

criterion_group!(
    ark_bench,
    bench_jubjub,
    bench_ed_on_bls_12_377,
    bench_bls12_381_g1,
    bench_bls12_381_g2,
    bench_bls12_377_g1,
    bench_bls12_377_g2
);
criterion_main!(ark_bench);

fn bench_jubjub(c: &mut Criterion) {
    let mut bench_group = c.benchmark_group("JubJub curve");

    let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);

    let mut base_point = ark_ed_on_bls12_381::EdwardsAffine::prime_subgroup_generator();
    let mut random_point =
        ark_ed_on_bls12_381::EdwardsAffine::from_random_bytes(bytes.as_ref()).unwrap();

    let bench_str = format!("random base mul");
    bench_group.bench_function(bench_str, move |b| {
        let r = ark_ed_on_bls12_381::Fr::rand(&mut rng);
        b.iter(|| {
            random_point.mul_assign(r);
        })
    });

    let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
    let bench_str = format!("fix base mul");
    bench_group.bench_function(bench_str, move |b| {
        let r = ark_ed_on_bls12_381::Fr::rand(&mut rng);
        b.iter(|| {
            base_point.mul_assign(r);
        })
    });
    bench_group.finish();
}

fn bench_ed_on_bls_12_377(c: &mut Criterion) {
    let mut bench_group = c.benchmark_group("ed_on_bls_12_377 curve");

    let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);

    let mut base_point = ark_ed_on_bls12_377::EdwardsAffine::prime_subgroup_generator();
    let mut random_point =
        ark_ed_on_bls12_377::EdwardsAffine::from_random_bytes(bytes.as_ref()).unwrap();

    let bench_str = format!("random base mul");
    bench_group.bench_function(bench_str, move |b| {
        let r = ark_ed_on_bls12_377::Fr::rand(&mut rng);
        b.iter(|| {
            random_point.mul_assign(r);
        })
    });

    let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
    let bench_str = format!("fix base mul");
    bench_group.bench_function(bench_str, move |b| {
        let r = ark_ed_on_bls12_377::Fr::rand(&mut rng);
        b.iter(|| {
            base_point.mul_assign(r);
        })
    });
    bench_group.finish();
}

fn bench_bls12_381_g1(c: &mut Criterion) {
    let mut bench_group = c.benchmark_group("BLS12-381 curve G1");

    let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);

    let base_point = ark_bls12_381::G1Affine::prime_subgroup_generator();

    let bench_str = format!("fix base mul");
    bench_group.bench_function(bench_str, move |b| {
        let r = ark_bls12_381::Fr::rand(&mut rng);
        b.iter(|| {
            let _ = base_point.mul(r);
        })
    });

    let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
    let r = ark_bls12_381::Fr::rand(&mut rng);
    let random_point = base_point.mul(r).into_affine();

    let bench_str = format!("random base mul");
    bench_group.bench_function(bench_str, move |b| {
        let r = ark_bls12_381::Fr::rand(&mut rng);
        b.iter(|| {
            let _ = random_point.mul(r);
        })
    });

    bench_group.finish();
}

fn bench_bls12_381_g2(c: &mut Criterion) {
    let mut bench_group = c.benchmark_group("BLS12-381 curve G2");

    let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);

    let base_point = ark_bls12_381::G2Affine::prime_subgroup_generator();

    let bench_str = format!("fix base mul");
    bench_group.bench_function(bench_str, move |b| {
        let r = ark_bls12_381::Fr::rand(&mut rng);
        b.iter(|| {
            let _ = base_point.mul(r);
        })
    });

    let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
    let r = ark_bls12_381::Fr::rand(&mut rng);
    let random_point = base_point.mul(r).into_affine();

    let bench_str = format!("random base mul");
    bench_group.bench_function(bench_str, move |b| {
        let r = ark_bls12_381::Fr::rand(&mut rng);
        b.iter(|| {
            let _ = random_point.mul(r);
        })
    });

    bench_group.finish();
}

fn bench_bls12_377_g1(c: &mut Criterion) {
    let mut bench_group = c.benchmark_group("BLS12-377 curve G1");

    let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);

    let base_point = ark_bls12_377::G1Affine::prime_subgroup_generator();

    let bench_str = format!("fix base mul");
    bench_group.bench_function(bench_str, move |b| {
        let r = ark_bls12_377::Fr::rand(&mut rng);
        b.iter(|| {
            let _ = base_point.mul(r);
        })
    });

    let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
    let r = ark_bls12_377::Fr::rand(&mut rng);
    let random_point = base_point.mul(r).into_affine();

    let bench_str = format!("random base mul");
    bench_group.bench_function(bench_str, move |b| {
        let r = ark_bls12_377::Fr::rand(&mut rng);
        b.iter(|| {
            let _ = random_point.mul(r);
        })
    });

    bench_group.finish();
}

fn bench_bls12_377_g2(c: &mut Criterion) {
    let mut bench_group = c.benchmark_group("BLS12-377 curve G2");

    let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);

    let base_point = ark_bls12_377::G2Affine::prime_subgroup_generator();

    let bench_str = format!("fix base mul");
    bench_group.bench_function(bench_str, move |b| {
        let r = ark_bls12_377::Fr::rand(&mut rng);
        b.iter(|| {
            let _ = base_point.mul(r);
        })
    });

    let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
    let r = ark_bls12_377::Fr::rand(&mut rng);
    let random_point = base_point.mul(r).into_affine();

    let bench_str = format!("random base mul");
    bench_group.bench_function(bench_str, move |b| {
        let r = ark_bls12_377::Fr::rand(&mut rng);
        b.iter(|| {
            let _ = random_point.mul(r);
        })
    });

    bench_group.finish();
}
