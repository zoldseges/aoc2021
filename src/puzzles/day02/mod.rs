pub fn name() -> Option<String> {
    Some(String::from("Dive!"))
}

pub fn solve_p1() -> Option<String> {
    let input = include_str!("input.txt");
    Some(get_solution_p1(input))
}

pub fn solve_p2() -> Option<String> {
    let input = include_str!("input.txt");
    Some(get_solution_p2(input))
}

fn get_solution_p1(input: &str) -> String {
    let mut pos = [0, 0];
    for line in input.lines() {
	let v: Vec<&str> = line.split(' ').collect();
	let dir = v[0];
	let val = v[1];
	match dir {
	    "forward" => pos[0] += val.parse::<i32>().unwrap(),
	    "down" => pos[1] += val.parse::<i32>().unwrap(),
	    "up" => pos[1] -= val.parse::<i32>().unwrap(),
	    _ => panic!("Direction {} is not handled", dir)
	}
    }
    String::from(format!("{}", pos[0] * pos[1]))
}

fn get_solution_p2(input: &str) -> String {
    let mut pos = [0, 0, 0];
    for line in input.lines() {
	let v: Vec<&str> = line.split(' ').collect();
	let dir = v[0];
	let val = v[1];
	let val = val.parse::<i32>().unwrap();
	match dir {
	    "forward" => { pos[0] += val;
			   pos[1] += pos[2] * val;
	    }
	    "down" => pos[2] += val,
	    "up" => pos[2] -= val,
	    _ => panic!("Direction {} is not handled", dir)
	}
    }
    String::from(format!("{}", pos[0] * pos[1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
	include_str!("input_test.txt")
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
