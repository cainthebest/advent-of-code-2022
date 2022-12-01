mod puzzles;
mod util;

fn main() {
    for puzzle in puzzles::get_all() {
        println!(
            "\x1b[93mRunning puzzle\x1b[0m: [ Year: {}, Name: {} ]",
            puzzle.year(),
            puzzle.name()
        );
        println!(
            "\x1B[32mResult\x1b[0m: [ Part A: {}, Part B: {} ]",
            puzzle.part_a(),
            puzzle.part_b()
        );
    }
}
