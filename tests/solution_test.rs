mod day1 {
    use aoc2021::puzzles::day01::*;

    #[test]
    fn solve_part1() {
	let input = include_str!("day1.txt");
	assert_eq!("7", get_solution_p1(input));
    }

    #[test]
    fn solve_part2() {
	let input = include_str!("day1.txt");
	assert_eq!("5", get_solution_p2(input));
    }
}

mod day2 {
    use aoc2021::puzzles::day02::*;

    #[test]
    fn solve_part1() {
	let input = include_str!("day2.txt");
	assert_eq!("150", get_solution_p1(input));
    }

    #[test]
    fn solve_part2() {
	let input = include_str!("day2.txt");
	assert_eq!("900", get_solution_p2(input));
    }
}
