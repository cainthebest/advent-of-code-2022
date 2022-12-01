mod puzzles;
mod util;

fn main() {
    for puzzle in puzzles::get_all() {
        println!("Running puzzle: {}", puzzle.name());
        println!("Part A: {}", puzzle.part_a());
        println!("Part B: {}", puzzle.part_b());
    }
}
