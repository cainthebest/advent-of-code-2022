use crate::util::AOC;

mod year_2022;

pub fn get_all() -> Vec<&'static dyn AOC> {
    let mut output = Vec::new();

    output.extend_from_slice(year_2022::ALL);

    output
}
