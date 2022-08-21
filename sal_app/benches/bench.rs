use api_actix_web::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("benches");
    g.measurement_time(std::time::Duration::from_secs(15));

    g.bench_function("make_user", |b| b.iter(|| black_box(make_user(15))));
    g.bench_function("get_users_1k", |b| b.iter(|| black_box(get_users(1000))));
    g.bench_function("get_users_10k", |b| b.iter(|| black_box(get_users(10_000))));
    g.bench_function("get_resp_1k", |b| b.iter(|| black_box(get_resp(1000))));
    g.bench_function("get_resp_10k", |b| b.iter(|| black_box(get_resp(10_000))));
    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
