mod utils;
pub mod puzzles;

mod get_puzzles;

pub struct Puzzle {
    day: i8,
    name: Option<String>,
    solve_p1: fn() -> Option<String>,
    solve_p2: fn() -> Option<String>,
    solution_p1: Option<String>,
    solution_p2: Option<String>
}

impl Puzzle {
    pub fn solve(&mut self) {
	self.solution_p1 = (self.solve_p1)();
	self.solution_p2 = (self.solve_p2)();
    }

    pub fn get_day(&self) -> &i8 {
	&self.day
    }

    pub fn get_name(&self) -> &Option<String> {
	&self.name
    }
    
    pub fn get_solution_part1(&self) -> &Option<String> {
	&self.solution_p1
    }

    pub fn get_solution_part2(&self) -> &Option<String> {
	&self.solution_p2
    }

}

pub fn init() -> Vec<Puzzle> {
    let vec = get_puzzles::get_puzzles();
    vec
}

