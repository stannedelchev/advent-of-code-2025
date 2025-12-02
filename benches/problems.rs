use advent2025::day01::Day01;
use advent2025::day02::Day02;
use advent2025::problem::ProblemLines;
use advent2025::problem::ProblemString;

use criterion::{criterion_group, criterion_main, Criterion};

pub fn day01(c: &mut Criterion) {
    let file = std::fs::read_to_string("inputs/input01.txt").unwrap();
    let problem = Day01 {};
    c.bench_function("Day 01 Part 1", |b| b.iter(|| problem.part1(file.lines())));
    c.bench_function("Day 01 Part 2", |b| b.iter(|| problem.part2(file.lines())));
}

pub fn day02(c: &mut Criterion) {
    let file = std::fs::read_to_string("inputs/input02.txt").unwrap();
    let problem = Day02 {};
    c.bench_function("Day 02 Part 1", |b| b.iter(|| problem.part1(&file)));
    c.bench_function("Day 02 Part 2", |b| b.iter(|| problem.part2(&file)));
}

criterion_group!(name = benches; config = Criterion::default().sample_size(100000); targets = day02);
criterion_main!(benches);
