use criterion::{Criterion, criterion_group, criterion_main};
use dofus_dmg_calculator::compute_dmg_estimation;
use dofus_dmg_calculator::compute_dmg_estimation_with_res;

fn benchmark_compute_dmg_estimation(c: &mut Criterion) {
    c.bench_function("compute_dmg_estimation", |b| {
        b.iter(|| compute_dmg_estimation(11, 1, 128, 3))
    });
}

fn benchmark_compute_dmg_estimation_with_res(c: &mut Criterion) {
    c.bench_function("compute_dmg_estimation_with_res", |b| {
        b.iter(|| compute_dmg_estimation_with_res(11, 1, 128, 3, 5, 30))
    });
}

criterion_group!(
    benches,
    benchmark_compute_dmg_estimation,
    benchmark_compute_dmg_estimation_with_res
);
criterion_main!(benches);
