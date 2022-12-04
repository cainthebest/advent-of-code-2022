use crate::util::{load_data_file, AOC};
use std::collections::HashSet;

fn score_item(char_: char) -> u8 {
    match char_ as u8 {
        97..=122 => char_ as u8 - 96,
        65..=90 => char_ as u8 - 38,
        _ => unreachable!(),
    }
}
pub struct Day03;

impl AOC for Day03 {
    fn name(&self) -> &'static str {
        "(Day 3): Rucksack Reorganization"
    }

    fn year(&self) -> u16 {
        2022
    }

    fn part_a(&self) -> String {
        let raw = load_data_file(2022, 3);
        let mut out = 0;

        for i in raw.trim().lines() {
            let mut bolth = i[0..i.len() / 2].chars().collect::<Vec<_>>();
            let pocket_2 = i[i.len() / 2..].chars().collect::<Vec<_>>();
            bolth.retain(|x| pocket_2.contains(x));
            bolth.dedup();

            debug_assert!(bolth.len() == 1);
            out += score_item(bolth[0]) as usize;
        }

        out.to_string()
    }

    fn part_b(&self) -> String {
        let raw = load_data_file(2022, 3);
        let mut out = 0;

        for i in raw.trim().lines().collect::<Vec<_>>().chunks(3) {
            let mut all = HashSet::new();
            i.iter().for_each(|x| all.extend(x.chars()));
            i.iter().for_each(|x| all.retain(|y| x.contains(*y)));

            debug_assert!(all.len() == 1);
            out += score_item(*all.iter().next().unwrap()) as usize;
        }

        out.to_string()
    }
}
