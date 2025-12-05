use crate::problem::ProblemLines;
use range_set_blaze::RangeSetBlaze;
use std::str::Lines;

pub struct Day05;

impl ProblemLines for Day05 {
    fn part1(&self, lines: Lines) -> String {
        let mut ranges = RangeSetBlaze::<u64>::new();
        let mut fresh = 0_u64;

        for line in lines {
            match line.split_once("-") {
                Some((left, right)) => {
                    let start = left.parse().unwrap();
                    let end = right.parse().unwrap();
                    ranges.ranges_insert(start..=end);
                }
                None => {
                    if let Ok(ingredient) = line.parse() && ranges.contains(ingredient) {
                        fresh += 1;
                    }
                }
            }
        }
        fresh.to_string()
    }

    fn part2(&self, lines: Lines) -> String {
        lines
            .map_while(|line| {
                line.split_once("-").map(|(left, right)| {
                    let start = left.parse().unwrap();
                    let end = right.parse().unwrap();
                    start..=end
                })
            })
            .collect::<RangeSetBlaze<u64>>()
            .into_ranges()
            .map(|r| r.count() as u64)
            .sum::<u64>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_example() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!("3", Day05 {}.part1(input.lines()));
    }

    #[test]
    fn part2_example() {
        let input = "3-5
10-14
16-20
12-18";
        assert_eq!("14", Day05 {}.part2(input.lines()));
    }
}
