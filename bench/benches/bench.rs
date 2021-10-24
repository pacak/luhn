use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_valid_isin(c: &mut Criterion) {
    let isin = b"US5949181045";
    c.bench_function("validate isin", |b| {
        b.iter(|| luhn3::valid(black_box(isin)))
    });
}

fn bench_valid_visa(c: &mut Criterion) {
    let visa = b"4111111111111111";
    c.bench_function("validate visa", |b| {
        b.iter(|| luhn3::decimal::valid(black_box(visa)))
    });
}

fn bench_valid_visa_vec(c: &mut Criterion) {
    let visa = b"4111111111111111";
    c.bench_function("validate visa vec", |b| {
        b.iter(|| unsafe { luhn3::decimal::valid_vec(black_box(visa)) })
    });
}

criterion_group!(
    benches,
    bench_valid_isin,
    bench_valid_visa,
    bench_valid_visa_vec
);
criterion_main!(benches);
