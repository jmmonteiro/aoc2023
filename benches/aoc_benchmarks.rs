use aoc2023::days::*;
use aoc2023::{utils::input::read_file, utils::structs::Solver};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn all_days_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("All Days");

    let vec = read_file("inputs/01/input.txt");
    group.bench_function("Day 01 : Part 1", |b| {
        b.iter(|| day01::Day.part1(black_box(&vec)))
    });
    group.bench_function("Day 01 : Part 2", |b| {
        b.iter(|| day01::Day.part2(black_box(&vec)))
    });

    group.finish();
}
criterion_group! {
    name = all_days;
    config = Criterion::default().significance_level(0.01).sample_size(500);
    targets = all_days_benchmark
}

criterion_main!(all_days);
