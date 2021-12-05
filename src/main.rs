use aoc2021::init as init;

fn main() {
    let mut vec = init();
    for mut puzzle in vec {
	puzzle.solve();
	if let (None, None) = (puzzle.get_solution_part1(), puzzle.get_solution_part2()) {
	    continue;
	}
	println!("Day {}", puzzle.get_day());
	match puzzle.get_name_part1() {
	    Some(n) => println!("\t{}", n),
	    None => println!("\tPart 1"),
	}
	match puzzle.get_solution_part1() {
	    Some(s) => println!("\t{}", s),
	    None => println!("\tunsolved"),
	}
	println!();
	match puzzle.get_name_part2() {
	    Some(n) => println!("\t{}", n),
	    None => println!("\tPart 2"),
	}
	match puzzle.get_solution_part2() {
	    Some(s) => println!("\t{}", s),
	    None => println!("\tunsolved"),
	}
    }
}
