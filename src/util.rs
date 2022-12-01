use std::fs;
pub trait AOC {
    fn name(&self) -> &'static str;
    fn part_a(&self) -> String;
    fn part_b(&self) -> String;
}

const YEARS: [u16; 1] = [2022];

pub fn load_data_file(year: u16, day: u8) -> String {
    match (day < 1 || day > 25, !YEARS.contains(&year)) {
            (true, true) => panic!(
                "Day must be between 1 and 25, You entered {}\nYear {:04} is not available, The available years are: {:?}",
                day, year, YEARS
            ),
            (false, true) => panic!(
                "Year {:04} is not available, The available years are: {:?}",
                year, YEARS
            ),
            (true, false) => panic!("Day must be between 1 and 25, You entered {}", day),
            _ => (),
    }

    let path = format!("data/{:04}/{:02}.txt", year, day);
    fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Failed to load file for {:04}/{:02}", year, day))
}
