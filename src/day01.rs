const INPUT: &str = include_str!("../input/day01.txt");

fn parse_supplies(input: &str) -> Vec<u32> {
    let mut output = input
        .replace('\r', "")
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum())
        .collect::<Vec<_>>();

    output.sort();
    output
}

pub(crate) fn part_one() -> String {
    let supplies = parse_supplies(INPUT);

    supplies.last().unwrap().to_string()
}

pub(crate) fn part_two() -> String {
    let supplies = parse_supplies(INPUT);

    let mut sum = 0;
    for i in supplies.iter().rev().take(3) {
        sum += i;
    }

    sum.to_string()
}
