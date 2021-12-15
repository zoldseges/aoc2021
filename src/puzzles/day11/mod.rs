use crate::utils::matrix::Matrix;

pub fn name() -> Option<String> {
    Some(String::from("Dumbo Octopus"))
}

pub fn solve_p1() -> Option<String> {
    let input = include_str!("input.txt");
    Some(get_solution_p1(input, 100).to_string())
}

pub fn solve_p2() -> Option<String> {
    let input = include_str!("input.txt");
    Some(get_solution_p2(input).to_string())
}

fn parse_input(input: &str) -> Vec<Vec<i8>> {
    let mut res = Vec::new();
    for line in input.lines() {
	let mut line_vec = Vec::new();
	for num in line.chars() {
	    line_vec.push(num.to_digit(10).unwrap() as i8);
	}
	res.push(line_vec);
    }
    res
}

fn get_solution_p1(input: &str, n: u32) -> u32 {
    let v = parse_input(input);
    let mut m = Matrix::from(v);
    let mut count = 0;
    for _ in 0..n {
	count += m.step();
    }
    count
}

fn get_solution_p2(input:&str) -> u32 {
    let mut count = 0;
    let v = parse_input(input);
    let mut m = Matrix::from(v);
    let size = m.get_rows() * m.get_cols();
    let size = size as u32;
    loop {
	count += 1;
	if m.step() == size { break; }
    }
    count
}

impl Matrix<i8> {
    
    fn flash(&mut self, c: usize, r: usize) {
	self.set(-1, c, r);
	let v = self.get_all_adjs(c, r);
	if let Some(v) = v {
	    for (c, r) in v {
		let n = self.get(c, r);
		if let Some(n) = n {
		    if n >= 0 {
			self.set(n+1, c, r);
			if n+1 > 9 {
			    self.flash(c, r)
			}
		    }
		}
	    }
	}
    }

    pub fn step(&mut self) -> u32{
	let mut count = 0;
	for r in 0..self.get_rows() {
	    for c in 0..self.get_cols() {
		let n = self.get(c, r).unwrap();
		self.set(n+1, c, r);
	    }
	}

	for r in 0..self.get_rows() {
	    for c in 0..self.get_cols() {
		let n = self.get(c, r).unwrap();
		if n >= 10 {
		    self.flash(c, r);
		}
	    }
	}

	for r in 0..self.get_rows() {
	    for c in 0..self.get_cols() {
		let n = self.get(c, r).unwrap();
		if n < 0 {
		    count += 1;
		    self.set(0, c, r);
		}
	    }
	}
	count
    }
}


#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    fn get_input() -> &'static str {
	include_str!("input_test.txt")
    }

    #[test]
    fn test_part_1() {
	let count = get_solution_p1(get_input(), 100);
	assert_eq!(1656, count);
    }

    #[test]
    fn test_part_2() {
	let count = get_solution_p2(get_input());
	assert_eq!(195, count);
    }
}
