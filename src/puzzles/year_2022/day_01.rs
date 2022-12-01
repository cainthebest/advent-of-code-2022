use crate::util::{load_data_file, AOC};

pub struct Day01;

fn parse_supplies(input: &str) -> Vec<u32> {
    let mut output = input
        .replace('\r', "")
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum())
        .collect::<Vec<_>>();

    output.sort();
    output
}

impl AOC for Day01 {
    fn name(&self) -> &'static str {
        "Day 01: Calorie Counting"
    }

    fn part_a(&self) -> String {
        let input = load_data_file(2022, 1);
        let supplies = parse_supplies(&input);

        supplies.last().unwrap().to_string()
    }

    fn part_b(&self) -> String {
        let input = load_data_file(2022, 1);
        let supplies = parse_supplies(&input);

        let mut sum = 0;
        for i in supplies.iter().rev().take(3) {
            sum += i;
        }

        sum.to_string()
    }
}
