use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day1::{part_one, part_two};

fn bench_part_one(c: &mut Criterion) {
    let data = std::fs::read("input.txt").unwrap();
    c.bench_function("part one", |b| b.iter(|| part_one(black_box(&data))));
}

fn bench_part_two(c: &mut Criterion) {
    let data = std::fs::read("input.txt").unwrap();
    c.bench_function("part two", |b| b.iter(|| part_two(black_box(&data))));
}

criterion_group!(benches, bench_part_one, bench_part_two);
criterion_main!(benches);
