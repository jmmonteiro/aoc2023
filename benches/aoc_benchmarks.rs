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

    let vec = read_file("inputs/02/input.txt");
    group.bench_function("Day 02 : Part 1", |b| {
        b.iter(|| day02::Day.part1(black_box(&vec)))
    });
    group.bench_function("Day 02 : Part 2", |b| {
        b.iter(|| day02::Day.part2(black_box(&vec)))
    });

    let vec = read_file("inputs/03/input.txt");
    group.bench_function("Day 03 : Part 1", |b| {
        b.iter(|| day03::Day.part1(black_box(&vec)))
    });
    group.bench_function("Day 03 : Part 2", |b| {
        b.iter(|| day03::Day.part2(black_box(&vec)))
    });

    let vec = read_file("inputs/04/input.txt");
    group.bench_function("Day 04 : Part 1", |b| {
        b.iter(|| day04::Day.part1(black_box(&vec)))
    });
    group.bench_function("Day 04 : Part 2", |b| {
        b.iter(|| day04::Day.part2(black_box(&vec)))
    });

    let vec = read_file("inputs/05/input.txt");
    group.bench_function("Day 05 : Part 1", |b| {
        b.iter(|| day04::Day.part1(black_box(&vec)))
    });
    // Too slow
    // group.bench_function("Day 04 : Part 2", |b| {
    // b.iter(|| day04::Day.part2(black_box(&vec)))
    // });

    let vec = read_file("inputs/06/input.txt");
    group.bench_function("Day 06 : Part 1", |b| {
        b.iter(|| day06::Day.part1(black_box(&vec)))
    });
    group.bench_function("Day 06 : Part 2", |b| {
        b.iter(|| day06::Day.part2(black_box(&vec)))
    });

    let vec = read_file("inputs/07/input.txt");
    group.bench_function("Day 07 : Part 1", |b| {
        b.iter(|| day07::Day.part1(black_box(&vec)))
    });
    group.bench_function("Day 07 : Part 2", |b| {
        b.iter(|| day07::Day.part2(black_box(&vec)))
    });

    group.finish();
}
criterion_group! {
    name = all_days;
    config = Criterion::default().significance_level(0.01).sample_size(500);
    targets = all_days_benchmark
}

criterion_main!(all_days);
