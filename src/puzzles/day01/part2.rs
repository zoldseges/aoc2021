use std::fs;

pub fn name() -> Option<String> {
    None
}

pub fn solve() -> Option<String> {
    let str = include_str!("input.txt");
    Some(get_solution(str))
}

pub fn get_solution(input: &str) -> String {
    let mut counter = 0;
    let mut lines = input.lines();
    let (mut a,mut b,mut c,mut d): (i32, i32, i32, i32);
    match lines.next() {
	Some(s) => a = s.parse().expect("Line couldn't be pased to int"),
	None => panic!("Input isn't correctly formed"),
    }
    match lines.next() {
	Some(s) => b = s.parse().expect("Line couldn't be pased to int"),
	None => panic!("Input isn't correctly formed"),
    }
    match lines.next() {
	Some(s) => c = s.parse().expect("Line couldn't be pased to int"),
	None => panic!("Input isn't correctly formed"),
    }
    for line in lines {
	let d: i32 = line.parse().expect("Line couldn't be parsed to int");
	if b+c+d > a+b+c {
	    counter += 1;
	}
	a = b;
	b = c;
	c = d;
    }
    counter.to_string()
}
