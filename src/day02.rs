use crate::problem::ProblemString;

pub struct Day02;

impl ProblemString for Day02 {
    fn part1(&self, str: &str) -> String {
        let invalid_ids = [1, 10, 100, 1000, 10000]
            .iter()
            .flat_map(|&power| {
                let start = power;
                let end = 10 * power;
                (start..end).map(move |i| i * end + i)
            })
            .collect::<Vec<i64>>();

        let mut sum = 0;
        for range in str.split(',') {
            let (start, end) = range
                .split_once('-')
                .map(|(start, end)| (start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap()))
                .unwrap();
            sum += invalid_ids
                .iter()
                .filter(|&i| start <= *i && *i <= end)
                .sum::<i64>();
        }
        sum.to_string()
    }

    fn part2(&self, str: &str) -> String {
        "".to_string()
    }
}
