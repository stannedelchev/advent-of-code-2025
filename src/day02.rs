use crate::problem::ProblemString;

pub struct Day02;

impl ProblemString for Day02 {
    fn part1(&self, str: &str) -> String {
        let invalid_ids = [1, 10, 100, 1000, 10000]
            .iter()
            .flat_map(|&start| {
                let end = 10 * start;
                (start..end).map(move |i| i * end + i)
            })
            .collect::<Vec<u64>>();

        let mut sum = 0;
        for range in str.split(',') {
            let (start, end) = range
                .split_once('-')
                .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
                .unwrap();
            sum += invalid_ids
                .iter()
                .filter(|&i| start <= *i && *i <= end)
                .sum::<u64>();
        }
        sum.to_string()
    }

    fn part2(&self, _str: &str) -> String {
        "".to_string()
    }
}
