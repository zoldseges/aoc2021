pub fn name() -> Option<String> {
    Some(String::from("Sonar sweep"))
}

pub fn solve_p1() -> Option<String> {
    let str = include_str!("input.txt");
    Some(get_solution_p1(str))
}

pub fn solve_p2() -> Option<String> {
    let str = include_str!("input.txt");
    Some(get_solution_p2(str))
}

fn get_solution_p1(input: &str) -> String {
    let mut counter = 0;
    let mut tmp;
    let mut lines = input.lines();
    match lines.next() {
	Some(s) => tmp = s.parse().expect("Line couldn't be pased to int"),
	None => panic!("Input isn't correctly formed"),
    }
    
    for line in lines {
	let line_as_num: i32 = line.parse().expect("Line couldn't be parsed to int");
	if line_as_num > tmp {
	    counter += 1;
	}
	tmp = line_as_num;
    }
    counter.to_string()
}

fn get_solution_p2(input: &str) -> String {
    let mut counter = 0;
    let mut lines = input.lines();
    let mut arr: [i32; 4] = [0; 4];
    match lines.next() {
	Some(s) => arr[0] = s.parse().expect("Line couldn't be pased to int"),
	None => panic!("Input isn't correctly formed"),
    }
    match lines.next() {
	Some(s) => arr[1] = s.parse().expect("Line couldn't be pased to int"),
	None => panic!("Input isn't correctly formed"),
    }
    match lines.next() {
	Some(s) => arr[2] = s.parse().expect("Line couldn't be pased to int"),
	None => panic!("Input isn't correctly formed"),
    }
    for line in lines {
	arr[3] = line.parse().expect("Line couldn't be parsed to int");
	if arr[1] + arr[2] + arr[3] > arr[0] + arr[1] + arr[2] {
	    counter += 1;
	}
	arr[0] = arr[1];
	arr[1] = arr[2];
	arr[2] = arr[3];
    }
    counter.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
	include_str!("input_test.txt")
    }

    #[test]
    fn test_part1() {
	assert_eq!("7", get_solution_p1(get_input()));
    }

    #[test]
    fn test_part2() {
	assert_eq!("5", get_solution_p2(get_input()));
    }
}
