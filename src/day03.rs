use crate::problem::ProblemLines;
use std::str::Lines;

pub struct Day03;

impl ProblemLines for Day03 {
    fn part1(&self, lines: Lines) -> String {
        lines
            .map(|line| {
                BatteriesIterator::new(line, 2)
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap()
            })
            .sum::<u64>()
            .to_string()
    }

    fn part2(&self, lines: Lines) -> String {
        lines
            .map(|line| {
                BatteriesIterator::new(line, 12)
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap()
            })
            .sum::<u64>()
            .to_string()
    }
}

struct BatteriesIterator<'a> {
    start: usize,
    end: usize,
    string: &'a str,
}

impl<'a> BatteriesIterator<'a> {
    fn new(string: &'a str, length: usize) -> Self {
        BatteriesIterator {
            start: 0,
            end: string.len() - length + 1,
            string,
        }
    }
}

impl Iterator for BatteriesIterator<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end > self.string.len() {
            return None;
        }

        let string = &self.string[self.start..self.end];

        // Can't use max_by_key() because it returns the last element instead of the first
        let (index, max) = string
            .char_indices()
            .fold((None, '\0'), |(index, max), (ix, c)| {
                if c > max {
                    return (Some(ix), c);
                }
                (index, max)
            });

        self.start += index.unwrap() + 1;
        self.end += 1;

        Some(max)
    }
}

#[cfg(test)]
mod tests {
    use crate::day03::BatteriesIterator;

    #[test]
    fn sample_part1() {
        let tests = [
            ("98", "987654321111111", 2),
            ("89", "811111111111119", 2),
            ("78", "234234234234278", 2),
            ("92", "818181911112111", 2),
        ];

        for (expected, string, length) in tests.into_iter() {
            let actual = BatteriesIterator::new(string, length).collect::<String>();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn sample_part2() {
        let tests = [
            ("987654321111", "987654321111111", 12),
            ("811111111119", "811111111111119", 12),
            ("434234234278", "234234234234278", 12),
            ("888911112111", "818181911112111", 12),
        ];

        for (expected, string, length) in tests.into_iter() {
            let actual = BatteriesIterator::new(string, length).collect::<String>();
            assert_eq!(expected, actual);
        }
    }
}
