mod puzzles;

mod get_puzzles;

pub struct Puzzle {
    name: Option<String>,
    day: i8,
    solve_p1: fn() -> Option<String>,
    solve_p2: fn() -> Option<String>,
    part1: Option<String>,
    part2: Option<String>
}

impl Puzzle {
    pub fn solve(&mut self) {
	self.part1 = (self.solve_p1)();
	self.part2 = (self.solve_p2)();
    }

    pub fn set_name(&mut self, name: Option<String>) {
	self.name = name;
    }

    pub fn get_part1(&self) -> &Option<String> {
	&self.part1
    }

    pub fn get_part2(&self) -> &Option<String> {
	&self.part2
    }

}

pub fn init() -> Vec<Puzzle> {
    let mut vec = get_puzzles::get_puzzles();
    vec
}
