use criterion::{black_box, criterion_group, criterion_main, Criterion};
use regex::Regex;

fn to_uppercase(s: &str) -> bool {
    s.to_uppercase() == "GBR"
}

fn regex_match(s: &str) -> bool {
    let re = Regex::new(r"(?i)GBR").unwrap();
    re.is_match(s)
}

// fn bench_to_uppercase(c: &mut Criterion) {
//     c.bench_function("to_uppercase GBR", |b| {
//         b.iter(|| to_uppercase(black_box("gBr")))
//     });
// }

// fn bench_regex_match(c: &mut Criterion) {
//     c.bench_function("regex_match GBR", |b| {
//         b.iter(|| regex_match(black_box("gBr")))
//     });
// }

// criterion_group!(benches, bench_to_uppercase, bench_regex_match);
// criterion_main!(benches);

fn bench_to_uppercase(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_uppercase");
    group.sample_size(500);
    group.bench_function("GBR", |b| b.iter(|| to_uppercase(black_box("gBr"))));
    group.finish();
}

fn bench_regex_match(c: &mut Criterion) {
    let mut group = c.benchmark_group("regex_match");
    group.sample_size(500);
    group.bench_function("GBR", |b| b.iter(|| regex_match(black_box("gBr"))));
    group.finish();
}

criterion_group!(benches, bench_to_uppercase, bench_regex_match);
criterion_main!(benches);
