use crate::problem::Problem;

mod day01;
mod problem;

fn main() {
    let problems: Vec<(Problem, &str, &str, &str, &str)> = vec![
    (
        Problem::Lines(Box::from(day01::Day01 {})),
        "Day01",
        "inputs/input01.txt",
        "765748",
        "27732508",
    )];
    for (problem, name, path, part1_solution, part2_solution) in problems {
            let file = std::fs::read_to_string(path).unwrap();

            let part1 = match &problem {
                Problem::Lines(p) => p.part1(file.lines()),
                Problem::String(p) => p.part1(&file),
            };
            let part1_correct = part1 == part1_solution;
            println!(
                "{} part 1: {} {}",
                name,
                part1,
                checkmark_or_cross(part1_correct)
            );

            let part2 = match &problem {
                Problem::Lines(p) => p.part2(file.lines()),
                Problem::String(p) => p.part2(&file),
            };
            let part2_correct = part2 == part2_solution;
            println!(
                "{} part 2: {} {}",
                name,
                part2,
                checkmark_or_cross(part2_correct)
            );
        }
    }

fn checkmark_or_cross(value: bool) -> char {
    if value {
        '\u{2705}'
    } else {
        '\u{274C}'
    }
}