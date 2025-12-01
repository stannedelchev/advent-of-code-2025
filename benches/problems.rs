use advent2025::day01::Day01;
use advent2025::problem::ProblemLines;

use criterion::{criterion_group, criterion_main, Criterion};

pub fn day01(c: &mut Criterion) {
    let file = std::fs::read_to_string("inputs/input01.txt").unwrap();
    let problem = Box::from(Day01 {});
    c.bench_function("Day 01 Part 1", |b| b.iter(|| problem.part1(file.lines())));
    c.bench_function("Day 01 Part 2", |b| b.iter(|| problem.part2(file.lines())));
}

criterion_group!(name = benches; config = Criterion::default().sample_size(100000); targets = day01);
criterion_main!(benches);
