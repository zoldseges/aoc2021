use std::fs;

pub fn name() -> Option<String> {
    Some(String::from("Sonar sweep"))
}

pub fn solve() -> Option<String> {
    let str = include_str!("input.txt");
    Some(get_solution(str))
}

pub fn get_solution(input: &str) -> String {
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
