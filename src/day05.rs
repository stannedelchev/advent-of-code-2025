use crate::problem::ProblemString;
use range_set_blaze::RangeSetBlaze;

pub struct Day05;

impl ProblemString for Day05 {
    fn part1(&self, str: &str) -> String {
        let (ranges, ingredients) = str.split_once("\n\n").unwrap();

        let ranges = ranges
            .lines()
            .map(|line| {
                let (left, right) = line.split_once("-").unwrap();
                let start = left.parse().unwrap();
                let end = right.parse().unwrap();
                start..=end
            })
            .collect::<RangeSetBlaze<u64>>();

        ingredients
            .lines()
            .filter(|line| {
                ranges.contains(line.parse::<u64>().unwrap())
            })
            .count()
            .to_string()
    }

    fn part2(&self, str: &str) -> String {
        str.split_once("\n\n")
            .unwrap()
            .0
            .lines()
            .map(|line| {
                let (left, right) = line.split_once("-").unwrap();
                let start = left.parse().unwrap();
                let end = right.parse().unwrap();
                start..=end
            })
            .collect::<RangeSetBlaze<u64>>()
            .ranges()
            .map(|r| r.count())
            .sum::<usize>()
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
        assert_eq!("3", Day05 {}.part1(input));
    }

    #[test]
    fn part2_example() {
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
        assert_eq!("14", Day05 {}.part2(input));
    }
}
