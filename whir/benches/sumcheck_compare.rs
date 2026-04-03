use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use p3_baby_bear::BabyBear;
use p3_field::extension::BinomialExtensionField;
use p3_multilinear_util::point::Point;
use p3_multilinear_util::poly::Poly;
use p3_whir::sumcheck::svo::{SvoAccumulatorStrategy, SvoClaim};
use rand::rngs::SmallRng;
use rand::{RngExt, SeedableRng};

type F = BabyBear;
type EF = BinomialExtensionField<F, 4>;

fn make_case(num_vars: usize, l: usize) -> (Poly<F>, Point<EF>) {
    let mut rng = SmallRng::seed_from_u64(((num_vars as u64) << 32) | l as u64);
    let poly = Poly::new((0..1 << num_vars).map(|_| rng.random()).collect());
    let point = Point::<EF>::rand(&mut rng, num_vars);
    (poly, point)
}

fn bench_svo_claim_new(c: &mut Criterion) {
    let mut group = c.benchmark_group("whir/sumcheck_svo_claim");
    group.sample_size(10);

    for &(num_vars, l) in &[(12, 3), (14, 4), (16, 5)] {
        let (poly, point) = make_case(num_vars, l);
        let label = format!("k{num_vars}_l{l}");

        group.bench_with_input(BenchmarkId::new("lagrange", &label), &label, |b, _| {
            b.iter(|| {
                SvoClaim::<F, EF>::new_with_strategy(
                    &point,
                    l,
                    &poly,
                    SvoAccumulatorStrategy::Lagrange,
                )
            });
        });

        group.bench_with_input(BenchmarkId::new("jolt", &label), &label, |b, _| {
            b.iter(|| {
                SvoClaim::<F, EF>::new_with_strategy(&point, l, &poly, SvoAccumulatorStrategy::Jolt)
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_svo_claim_new);
criterion_main!(benches);
