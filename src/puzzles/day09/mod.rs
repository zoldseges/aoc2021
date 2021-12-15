use std::collections::HashSet;
use crate::utils::matrix::Matrix;

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
	    walk_basin(m, m.get_close_adjs(c, r).unwrap(), basin);
	}
    }
}

fn get_low_points(m: &Matrix<u8>) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for r in 0..m.get_rows() {
	for c in 0..m.get_cols() {
	    let mut is_low = true;
	    for (a_c, a_r) in m.get_close_adjs(c, r).unwrap() {
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
