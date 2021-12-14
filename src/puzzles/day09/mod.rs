use std::collections::HashSet;
use matrix::Matrix;

pub fn name() -> Option<String> {
    Some(String::from("Smoke Basin"))
}

pub fn solve_p1() -> Option<String> {
    let input = include_str!("input.txt");
    Some(get_solution_p1(input).to_string())
}

pub fn solve_p2() -> Option<String> {
    let input = include_str!("input.txt");
    Some(get_solution_p2(input).to_string())
}

fn get_solution_p1(input: &str) -> u32 {
    let mut res = 0;
    let vec = parse_input(input);
    let m = Matrix::from(vec);
    let lows = get_low_points(&m).
	into_iter().
	map(|x| { m.get(x.0, x.1).unwrap() } );
    for p in lows {
	res += 1 + p as u32;
    }
    res
}

fn get_solution_p2(input: &str) -> u32 {
    let vec = parse_input(input);
    let m = Matrix::from(vec);
    let basins = get_basins(m);
    let mut basin_sizes: Vec<u32> = basins.into_iter().map(|x| { x.size() as u32 }).collect();
    basin_sizes.sort();
    basin_sizes.reverse();
    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut res = Vec::new();
    for line in input.lines() {
	let mut line_vec = Vec::new();
	for num in line.chars() {
	    line_vec.push(num.to_digit(10).unwrap() as u8);
	}
	res.push(line_vec);
    }
    res
}

struct Basin {
    cords: HashSet<(usize, usize)>,
}

impl Basin {
    pub fn insert(&mut self, cord: (usize, usize)) -> bool {
	self.cords.insert(cord)
    }

    pub fn size(&self) -> usize {
	self.cords.len()
    }
}


fn get_basins(mut m: Matrix<u8>) -> Vec<Basin> {
    let mut res = Vec::new();
    for r in 0..m.get_rows() {
	for c in 0..m.get_cols() {
	    if m.get(c, r) < Some(9) {
		let mut basin = Basin { cords: HashSet::new() };
		walk_basin(&mut m, vec![(c, r)], &mut basin);
		res.push(basin)
	    }
	}
    }
    res
}

fn walk_basin(m: &mut Matrix<u8>, cords: Vec<(usize, usize)>, basin: &mut Basin) {
    for (c, r) in cords {
	if m.get(c, r) < Some(9) {
	    basin.insert((c, r));
	    m.set(10, c, r);
	    walk_basin(m, m.get_adjs(c, r).unwrap(), basin);
	}
    }
}

fn get_low_points(m: &Matrix<u8>) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for r in 0..m.get_rows() {
	for c in 0..m.get_cols() {
	    let mut is_low = true;
	    for (a_c, a_r) in m.get_adjs(c, r).unwrap() {
		if m.get(a_c, a_r) <= m.get(c, r) {
		    is_low = false;
		    break;
		}
	    }
	    if is_low { res.push((c, r)); }
	}
    }
    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    fn get_input() -> &'static str {
	include_str!("input_test.txt")
    }

    #[test]
    fn test_matrix() {
	let vec = parse_input(get_input());
	let matrix = Matrix::from(vec);
	assert_eq!(Some(7), matrix.get(4, 2));
	assert_eq!(None, matrix.get(1, 99));
    }

    #[test]
    fn test_low_points() {
	let vec = parse_input(get_input());
	let matrix = Matrix::from(vec);
	assert_eq!(vec![(1, 0),
			(9, 0),
			(2, 2),
			(6, 4)], get_low_points(&matrix));
    }

    #[test]
    fn test_part_1() {
	assert_eq!(15, get_solution_p1(get_input()));
    }

    #[test]
    fn test_get_basins() {
	let vec = parse_input(get_input());
	let m = Matrix::from(vec);
	let basins = get_basins(m);
	assert_eq!(4, basins.len());
    }

    #[test]
    fn test_part_2() {
	assert_eq!(1134, get_solution_p2(get_input()));
    }
}

pub mod matrix {

    pub struct Matrix<T> {
	cols: usize,
	rows: usize,
	vec: Vec<T>,
    }

    impl<T: Copy> Matrix<T> {

	pub fn from(from: Vec<Vec<T>>) -> Matrix<T> {
	    let mut vec = Vec::new();
	    let cols = from[0].len();
	    let rows = from.len();
	    for row in from {
		vec.extend(row);
	    }
	    Matrix { cols, rows, vec }
	}

	fn is_valid(&self, col: usize, row: usize) -> bool {
	    if col < self.cols &&
		row < self.rows{
		    true
		}
	    else {
		false
	    }
	}

	pub fn get(&self, col: usize, row: usize) -> Option<T> {
	    if self.is_valid(col, row)
	    {
		Some(self.vec[col + row * self.cols])
	    } else {
		None
	    }
	}

	pub fn set(&mut self, val: T, col: usize, row: usize) {
	    if self.is_valid(col, row) {
		self.vec[col + row * self.cols] = val;
	    }
	}
	
	pub fn get_rows(&self) -> usize {
	    self.rows
	}

	pub fn get_cols(&self) -> usize {
	    self.cols
	}

	// for skipping checks we use the checks found in the get method
	// we assume that the matrix is smaller then usize
	// out of bound requests return empty vector
	pub fn get_adjs(&self, col: usize, row: usize) -> Option<Vec<(usize, usize)>> {
	    if self.is_valid(col, row) {
		let mut res = Vec::new();
		let adj_positions = [(col, row.wrapping_sub(1)),
				     (col + 1, row),
				     (col, row + 1),
				     (col.wrapping_sub(1), row)];
		for pos in adj_positions {
		    if self.is_valid(pos.0, pos.1) { res.push(pos) };
		}
		return Some(res);
	    } else {
		return None;
	    }
	}
    }
    
    #[cfg(test)]
    mod tests {
	use super::*;

	fn get_matrix() -> Matrix<u8> {
	    let vec = vec![vec![2,1,9,9,9,4,3,2,1,0],
			   vec![3,9,8,7,8,9,4,9,2,1],
			   vec![9,8,5,6,7,8,9,8,9,2],
			   vec![8,7,6,7,8,9,6,7,8,9],
			   vec![9,8,9,9,9,6,5,6,7,8],];
	    Matrix::from(vec)
	}

	#[test]
	fn test_adjecency() {
	    let m = get_matrix();
	    assert_eq!(Some(vec![(1, 0) ,(0, 1)]), m.get_adjs(0, 0));
	    assert_eq!(Some(vec![(6, 1),
				 (7, 2),
				 (6, 3),
				 (5, 2)]), m.get_adjs(6, 2));
	    assert_eq!(Some(vec![(4, 3),
				 (5, 4),
				 (3, 4)]), m.get_adjs(4,4)); 
	    assert_eq!(None, m.get_adjs(3,5));
	    assert_eq!(None, m.get_adjs(10,3));
	}
    }
}
