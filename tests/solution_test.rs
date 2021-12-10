mod day1 {
    use aoc2021::puzzles::day01::*;

    fn get_input() -> &'static str{
	include_str!("day1.txt")
    }

    #[test]
    fn test_part1() {
	assert_eq!("7", get_solution_p1(get_input()));
    }

    #[test]
    fn test_part2() {
	assert_eq!("5", get_solution_p2(get_input()));
    }
}

mod day2 {
    use aoc2021::puzzles::day02::*;

    fn get_input() -> &'static str{
	include_str!("day2.txt")
    }

    #[test]
    fn test_part1() {
	assert_eq!("150", get_solution_p1(get_input()));
    }

    #[test]
    fn test_part2() {
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
    fn test_part1() {
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

mod day4 {
    use aoc2021::puzzles::day04::*;

    fn get_input() -> &'static str{
	include_str!("day4.txt")
    }

    #[test]
    fn test_get_draws() {
	let (draws, _, _) = read_input(get_input());
	assert_eq!(vec!(7,4,9,5,11,17,23,2,0,14,21,
		       24,10,16,13,6,15,25,12,22,
		       18,20,8,19,3,26,1), draws);
    }

    #[test]
    fn test_hashmap() {
	let (_, _, hashmap) = read_input(get_input());
	assert_eq!(3, hashmap.get(&24).unwrap().len());
	assert!(hashmap.get(&24).unwrap().contains(&(0, 1, 4)));
	assert!(hashmap.get(&24).unwrap().contains(&(1, 3, 3)));
	assert!(hashmap.get(&24).unwrap().contains(&(2, 0, 3)));
	assert!(hashmap.get(&72).is_none());
	assert!(!hashmap.get(&24).unwrap().contains(&(2,2,2)));
	assert!(!hashmap.get(&24).unwrap().contains(&(2,2,8)));
	assert!(!hashmap.get(&24).unwrap().contains(&(5,2,1)));
    }

    #[test]
    fn test_first_winning_last_draw() {
	let (_, l_draw) = run(true, read_input(get_input()));
	assert_eq!(24, l_draw);
    }
    
    #[test]
    fn test_first_winning_sum() {
	let (sum, _) = run(true, read_input(get_input()));
	assert_eq!(188, sum);
    }

    #[test]
    fn test_last_winning_last_draw() {
	let (_, l_draw) = run(false, read_input(get_input()));
	assert_eq!(13, l_draw);
    }
    
    #[test]
    fn test_last_winning_sum() {
	let (sum, _) = run(false, read_input(get_input()));
	assert_eq!(148, sum);
    }
}

mod day5 {
    use aoc2021::puzzles::day05::*;

    fn get_input() -> &'static str{
	include_str!("day5.txt")
    }

    #[test]
    fn test_str_to_line() {
	let s = "15,2 -> 8,9";
	let l = Line::new(15, 2, 8, 9);
	assert_eq!(l, str_to_line(s));
    }

    #[test]
    fn test_part1() {
	assert_eq!(5, get_solution(true, get_input()));
    }
    
    #[test]
    fn test_part2() {
	assert_eq!(12, get_solution(false, get_input()));
    }
}
