use advent2025::day01::Day01;
use advent2025::day02::Day02;
use advent2025::day03::Day03;
use advent2025::day04::Day04;
use advent2025::problem::{ProblemLines, ProblemString};

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

pub fn day03(c: &mut Criterion) {
    let file = std::fs::read_to_string("inputs/input03.txt").unwrap();
    let problem = Day03 {};
    c.bench_function("Day 03 Part 1", |b| b.iter(|| problem.part1(file.lines())));
    c.bench_function("Day 03 Part 2", |b| b.iter(|| problem.part2(file.lines())));
}

pub fn day04(c: &mut Criterion) {
    let file = std::fs::read_to_string("inputs/input04.txt").unwrap();
    let problem = Day04 {};
    c.bench_function("Day 04 Part 1", |b| b.iter(|| problem.part1(file.lines())));
    c.bench_function("Day 04 Part 2", |b| b.iter(|| problem.part2(file.lines())));
}

criterion_group!(name = benches; config = Criterion::default().sample_size(1000); targets = day04);
criterion_main!(benches);
