use crate::util::{load_data_file, AOC};

fn parse_supplies(input: &str) -> Vec<u32> {
    let mut output: Vec<u32> = input
        .replace('\r', "")
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum())
        .collect::<Vec<_>>();

    output.sort();
    output
}

pub struct Day01;

impl AOC for Day01 {
    fn name(&self) -> &'static str {
        "(Day 1): Calorie Counting"
    }

    fn year(&self) -> u16 {
        2022
    }

    fn part_a(&self) -> String {
        let input: String = load_data_file(2022, 1);
        let supplies: Vec<u32> = parse_supplies(&input);

        supplies.last().unwrap().to_string()
    }

    fn part_b(&self) -> String {
        let input: String = load_data_file(2022, 1);
        let supplies: Vec<u32> = parse_supplies(&input);

        let mut sum: u32 = 0;
        for i in supplies.iter().rev().take(3) {
            sum += i;
        }

        sum.to_string()
    }
}
