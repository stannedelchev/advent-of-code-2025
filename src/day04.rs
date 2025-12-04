use crate::problem::ProblemLines;
use itertools::Itertools;
use std::str::Lines;

pub struct Day04;

impl ProblemLines for Day04 {
    fn part1(&self, lines: Lines) -> String {
        let floor = Floor::new(lines);
        floor.accessible_rolls().len().to_string()
    }

    fn part2(&self, lines: Lines) -> String {
        let mut floor = Floor::new(lines);

        let mut total_removed = 0;
        let mut accessible = floor.accessible_rolls();
        while accessible.len() > 0 {
            total_removed += accessible.len();
            floor.remove_rolls(accessible);
            accessible = floor.accessible_rolls();
        }

        total_removed.to_string()
    }
}

struct Floor {
    grid: Vec<Vec<Option<PaperRoll>>>,
}

struct PaperRoll();

struct FloorCoordinates {
    row: usize,
    column: usize,
}

impl Floor {
    fn new(lines: Lines) -> Floor {
        let grid: Vec<Vec<Option<PaperRoll>>> = lines
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '@' => Some(PaperRoll {}),
                        _ => None,
                    })
                    .collect_vec()
            })
            .collect_vec();
        Floor { grid }
    }

    fn accessible_rolls(&self) -> Vec<FloorCoordinates> {
        let grid = &self.grid;
        let mut accessible_rolls = vec![];

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col].is_none() {
                    continue;
                }

                let mut neighbours = 0;
                // Check upper row
                if row > 0 {
                    if col > 0 && grid[row - 1][col - 1].is_some() {
                        neighbours += 1;
                    }
                    if grid[row - 1][col].is_some() {
                        neighbours += 1;
                    }
                    if col < grid[row].len() - 1 && grid[row - 1][col + 1].is_some() {
                        neighbours += 1;
                    }
                }

                // Check same row
                {
                    if col > 0 && grid[row][col - 1].is_some() {
                        neighbours += 1;
                    }
                    if col < grid[row].len() - 1 && grid[row][col + 1].is_some() {
                        neighbours += 1;
                    }
                }

                // Check row below
                if row < grid.len() - 1 {
                    if col > 0 && grid[row + 1][col - 1].is_some() {
                        neighbours += 1;
                    }
                    if grid[row + 1][col].is_some() {
                        neighbours += 1;
                    }
                    if col < grid[row].len() - 1 && grid[row + 1][col + 1].is_some() {
                        neighbours += 1;
                    }
                }

                if neighbours < 4 {
                    accessible_rolls.push(FloorCoordinates { row, column: col });
                }
            }
        }

        accessible_rolls
    }

    fn remove_rolls(&mut self, rolls: Vec<FloorCoordinates>) {
        for coords in rolls {
            self.grid[coords.row][coords.column] = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!("13", Day04 {}.part1(input.lines()));
    }

    #[test]
    fn test_part2() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!("43", Day04 {}.part2(input.lines()));
    }
}
