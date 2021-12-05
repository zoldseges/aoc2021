use crate::puzzles;

use crate::Puzzle;

pub fn get_puzzles() -> Vec<Puzzle> {
    let mut vec = Vec::new();
    let mut puzzle = Puzzle {
	name: puzzles::day01::name(),
	day: 1,
	solve_p1: puzzles::day01::solve_part1,
	solve_p2: puzzles::day01::solve_part2,
	part1: None,
	part2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	name: puzzles::day02::name(),
	day: 2,
	solve_p1: puzzles::day02::solve_part1,
	solve_p2: puzzles::day02::solve_part2,
	part1: None,
	part2: None,
    };
    vec.push(puzzle);
    vec
}
