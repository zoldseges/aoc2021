use crate::utils::matrix::Matrix;

pub fn name() -> Option<String> {
    None
}

pub fn solve_p1() -> Option<String> {
    None
}

pub fn solve_p2() -> Option<String> {
    None
}

fn get_solution_p1(input: &str) -> u32{
    let input = parse_input(input);
    99
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

fn count_low_points<T>(m: Matrix<T>) -> u32 {
    9
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
	assert_eq!(15, get_solution_p1(get_input()));
    }

    #[test]
    fn test_matrix() {
	let vec = parse_input(get_input());
	let matrix = Matrix::from(vec);
	assert_eq!(Some(7), matrix.get(4, 2));
	assert_eq!(None, matrix.get(1, 99));
    }

    
}
