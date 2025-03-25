use criterion::{Criterion, criterion_group, criterion_main};
use dofus_dmg_calculator::compute_dmg_estimation;

fn benchmark_function(c: &mut Criterion) {
    c.bench_function("compute_dmg_estimation", |b| {
        b.iter(|| compute_dmg_estimation(11, 1, 128, 3))
    });
}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);
