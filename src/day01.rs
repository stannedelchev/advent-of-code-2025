use crate::problem::ProblemLines;
use std::str::Lines;

pub struct Day01;

impl ProblemLines for Day01 {
    fn part1(&self, lines: Lines) -> String {
        let mut clicks = 0;
        let mut dial = 50;

        for line in lines {
            let direction = if line.chars().nth(0).unwrap() == 'L' {
                -1
            } else {
                1
            };

            let value = line[1..].parse::<i32>().unwrap();
            for _ in 0..value {
                dial += direction;
                if dial == 100 {
                    dial = 0;
                } else if dial == -1 {
                    dial = 99;
                }
            }

            if dial == 0 {
                clicks += 1;
            }
        }

        clicks.to_string()
    }

    fn part2(&self, lines: Lines) -> String {
        let mut clicks = 0;
        let mut dial = 50;

        for line in lines {
            let direction = if line.chars().nth(0).unwrap() == 'L' {
                -1
            } else {
                1
            };

            let value = line[1..].parse::<i32>().unwrap();
            for _ in 0..value {
                dial += direction;

                if dial == 100 {
                    dial = 0;
                } else if dial == -1 {
                    dial = 99;
                }

                if dial == 0 {
                    clicks += 1;
                }
            }
        }

        clicks.to_string()
    }
}