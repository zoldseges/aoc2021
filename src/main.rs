use aoc2021::init as init;

fn main() {
    let mut vec = init();
    for mut puzzle in vec {
	puzzle.solve();
	println!("{:?}", puzzle.get_part1());
	println!("{:?}", puzzle.get_part2());
    }
}
