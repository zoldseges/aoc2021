mod day1 {
    use aoc2021::puzzles::day01::*;

    fn get_input() -> &'static str{
	include_str!("day1.txt")
    }

    #[test]
    fn solve_part1() {
	assert_eq!("7", get_solution_p1(get_input()));
    }

    #[test]
    fn solve_part2() {
	assert_eq!("5", get_solution_p2(get_input()));
    }
}

mod day2 {
    use aoc2021::puzzles::day02::*;

    fn get_input() -> &'static str{
	include_str!("day2.txt")
    }

    #[test]
    fn solve_part1() {
	assert_eq!("150", get_solution_p1(get_input()));
    }

    #[test]
    fn solve_part2() {
	assert_eq!("900", get_solution_p2(get_input()));
    }
}

mod day3 {
    use aoc2021::puzzles::day03::*;

    fn get_input() -> &'static str{
	include_str!("day3.txt")
    }

    #[test]
    fn test_bin_to_vec() {
	assert_eq!(58, bin_vec_to_dec(vec!(0,0,0,1,1,1,0,1,0)));
    }

    #[test]
    fn solve_part1() {
	assert_eq!("198", get_solution_p1(get_input()));
    }

    #[test]
    fn test_node_add() {
	let mut root = Node::new();
	root.add_node(0);
	root.get_side_node(0).inc_count();
	assert_eq!(root.get_side_node(0).get_count(), 1);
    }

    #[test]
    fn test_add_str_to_tree() {
	let mut res_vec = vec!();
	let mut node = &mut Node::new();
	node.add_str_to_tree("1001");
	node.add_str_to_tree("1011");
	res_vec.push(node.get_count());
	node = node.get_side_node(1);
	res_vec.push(node.get_count());
	node = node.get_side_node(0);
	res_vec.push(node.get_count());
	node = node.get_side_node(0);
	res_vec.push(node.get_count());
	node = node.get_side_node(1);
	res_vec.push(node.get_count());
	assert_eq!(vec!(2,2,2,1,1), res_vec);
    }

    #[test]
    fn test_part_oxy() {
	assert_eq!(23, get_part_p2(get_input(), true));
    }

    #[test]
    fn test_part_co() {
	assert_eq!(10, get_part_p2(get_input(), false));
    }
}
