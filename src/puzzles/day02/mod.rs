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

pub fn get_solution_p1(input: &str) -> String {
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

pub fn get_solution_p2(input: &str) -> String {
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
