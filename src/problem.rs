use std::str::Lines;

pub enum Problem {
    Lines(Box<dyn ProblemLines>),
    String(Box<dyn ProblemString>),
}

pub trait ProblemLines {
    fn part1(&self, lines: Lines) -> String;
    fn part2(&self, lines: Lines) -> String;
}

pub trait ProblemString {
    fn part1(&self, str: &str) -> String;
    fn part2(&self, str: &str) -> String;
}
