use criterion::{Criterion, criterion_group, criterion_main};
use test_bin::get_test_bin;

fn benchmark_function(c: &mut Criterion) {
    c.bench_function("dofus-dmg-calculator", |b| {
        b.iter(|| {
            let mut cmd = get_test_bin("dofus-dmg-calculator");

            cmd.args(["-i", "9"])
                .args(["-j", "11"])
                .args(["-k", "12"])
                .args(["-l", "14"])
                .args(["-s", "128"])
                .args(["-p", "12"])
                .args(["-f", "1"])
                .args(["-x", "3"])
                .args(["-r", "45"]);

            cmd.output()
        })
    });
}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);
