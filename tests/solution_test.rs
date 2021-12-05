mod day1 {
    use aoc2021::puzzles::day01::{part1 as part1, part2 as part2};

    #[test]
    fn solve_part1() {
	let input = include_str!("day1.txt");
	assert_eq!("7", part1::get_solution(input));
    }

    #[test]
    fn solve_part2() {
	let input = include_str!("day1.txt");
	assert_eq!("5", part2::get_solution(input));
    }

}
