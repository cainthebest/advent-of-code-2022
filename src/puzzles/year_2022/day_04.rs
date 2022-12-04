use crate::util::{load_data_file, AOC};

fn assignment_loop(raw: String) -> Vec<((u32, u32), (u32, u32))> {
    raw.trim()
        .lines()
        .map(|x| x.split_once(',').unwrap())
        .map(|(a, b)| (split_range(a), split_range(b)))
        .collect()
}

fn split_range(range: &str) -> (u32, u32) {
    let mut range = range.split('-').map(|x| x.parse::<u32>().unwrap());
    (range.next().unwrap(), range.next().unwrap())
}
pub struct Day04;

impl AOC for Day04 {
    fn name(&self) -> &'static str {
        "(Day 4): Camp Cleanup"
    }

    fn year(&self) -> u16 {
        2022
    }

    fn part_a(&self) -> String {
        let raw = load_data_file(2022, 4);
        let mut out = 0;

        for (p1, p2) in assignment_loop(raw) {
            out += ((p1.0 >= p2.0 && p1.1 <= p2.1) || (p2.0 >= p1.0 && p2.1 <= p1.1)) as usize;
        }

        out.to_string()
    }

    fn part_b(&self) -> String {
        let raw = load_data_file(2022, 4);
        let mut out = 0;

        for (p1, p2) in assignment_loop(raw) {
            out += (p1.0.max(p2.0) <= p1.1.min(p2.1)) as usize;
        }

        out.to_string()
    }
}
