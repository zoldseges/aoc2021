use crate::puzzles;

use crate::Puzzle;

pub fn get_puzzles() -> Vec<Puzzle> {
    let mut vec = Vec::new();
    let mut puzzle = Puzzle {
	day: 1,
	name_p1: puzzles::day01::part1::name(),
	name_p2: puzzles::day01::part2::name(),
	solve_p1: puzzles::day01::part1::solve,
	solve_p2: puzzles::day01::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 2,
	name_p1: puzzles::day02::part1::name(),
	name_p2: puzzles::day02::part2::name(),
	solve_p1: puzzles::day02::part1::solve,
	solve_p2: puzzles::day02::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 3,
	name_p1: puzzles::day03::part1::name(),
	name_p2: puzzles::day03::part2::name(),
	solve_p1: puzzles::day03::part1::solve,
	solve_p2: puzzles::day03::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 4,
	name_p1: puzzles::day04::part1::name(),
	name_p2: puzzles::day04::part2::name(),
	solve_p1: puzzles::day04::part1::solve,
	solve_p2: puzzles::day04::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 5,
	name_p1: puzzles::day05::part1::name(),
	name_p2: puzzles::day05::part2::name(),
	solve_p1: puzzles::day05::part1::solve,
	solve_p2: puzzles::day05::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 6,
	name_p1: puzzles::day06::part1::name(),
	name_p2: puzzles::day06::part2::name(),
	solve_p1: puzzles::day06::part1::solve,
	solve_p2: puzzles::day06::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 7,
	name_p1: puzzles::day07::part1::name(),
	name_p2: puzzles::day07::part2::name(),
	solve_p1: puzzles::day07::part1::solve,
	solve_p2: puzzles::day07::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 8,
	name_p1: puzzles::day08::part1::name(),
	name_p2: puzzles::day08::part2::name(),
	solve_p1: puzzles::day08::part1::solve,
	solve_p2: puzzles::day08::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 9,
	name_p1: puzzles::day09::part1::name(),
	name_p2: puzzles::day09::part2::name(),
	solve_p1: puzzles::day09::part1::solve,
	solve_p2: puzzles::day09::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 10,
	name_p1: puzzles::day10::part1::name(),
	name_p2: puzzles::day10::part2::name(),
	solve_p1: puzzles::day10::part1::solve,
	solve_p2: puzzles::day10::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 10,
	name_p1: puzzles::day10::part1::name(),
	name_p2: puzzles::day10::part2::name(),
	solve_p1: puzzles::day10::part1::solve,
	solve_p2: puzzles::day10::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 11,
	name_p1: puzzles::day11::part1::name(),
	name_p2: puzzles::day11::part2::name(),
	solve_p1: puzzles::day11::part1::solve,
	solve_p2: puzzles::day11::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 12,
	name_p1: puzzles::day12::part1::name(),
	name_p2: puzzles::day12::part2::name(),
	solve_p1: puzzles::day12::part1::solve,
	solve_p2: puzzles::day12::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 13,
	name_p1: puzzles::day13::part1::name(),
	name_p2: puzzles::day13::part2::name(),
	solve_p1: puzzles::day13::part1::solve,
	solve_p2: puzzles::day13::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 14,
	name_p1: puzzles::day14::part1::name(),
	name_p2: puzzles::day14::part2::name(),
	solve_p1: puzzles::day14::part1::solve,
	solve_p2: puzzles::day14::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 15,
	name_p1: puzzles::day15::part1::name(),
	name_p2: puzzles::day15::part2::name(),
	solve_p1: puzzles::day15::part1::solve,
	solve_p2: puzzles::day15::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 16,
	name_p1: puzzles::day16::part1::name(),
	name_p2: puzzles::day16::part2::name(),
	solve_p1: puzzles::day16::part1::solve,
	solve_p2: puzzles::day16::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 17,
	name_p1: puzzles::day17::part1::name(),
	name_p2: puzzles::day17::part2::name(),
	solve_p1: puzzles::day17::part1::solve,
	solve_p2: puzzles::day17::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 18,
	name_p1: puzzles::day18::part1::name(),
	name_p2: puzzles::day18::part2::name(),
	solve_p1: puzzles::day18::part1::solve,
	solve_p2: puzzles::day18::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 19,
	name_p1: puzzles::day19::part1::name(),
	name_p2: puzzles::day19::part2::name(),
	solve_p1: puzzles::day19::part1::solve,
	solve_p2: puzzles::day19::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 20,
	name_p1: puzzles::day20::part1::name(),
	name_p2: puzzles::day20::part2::name(),
	solve_p1: puzzles::day20::part1::solve,
	solve_p2: puzzles::day20::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 21,
	name_p1: puzzles::day21::part1::name(),
	name_p2: puzzles::day21::part2::name(),
	solve_p1: puzzles::day21::part1::solve,
	solve_p2: puzzles::day21::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 22,
	name_p1: puzzles::day22::part1::name(),
	name_p2: puzzles::day22::part2::name(),
	solve_p1: puzzles::day22::part1::solve,
	solve_p2: puzzles::day22::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 23,
	name_p1: puzzles::day23::part1::name(),
	name_p2: puzzles::day23::part2::name(),
	solve_p1: puzzles::day23::part1::solve,
	solve_p2: puzzles::day23::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 24,
	name_p1: puzzles::day24::part1::name(),
	name_p2: puzzles::day24::part2::name(),
	solve_p1: puzzles::day24::part1::solve,
	solve_p2: puzzles::day24::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 25,
	name_p1: puzzles::day25::part1::name(),
	name_p2: puzzles::day25::part2::name(),
	solve_p1: puzzles::day25::part1::solve,
	solve_p2: puzzles::day25::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    let mut puzzle = Puzzle {
	day: 25,
	name_p1: puzzles::day25::part1::name(),
	name_p2: puzzles::day25::part2::name(),
	solve_p1: puzzles::day25::part1::solve,
	solve_p2: puzzles::day25::part2::solve,
	solution_p1: None,
	solution_p2: None,
    };
    vec.push(puzzle);
    vec
}
