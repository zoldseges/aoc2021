pub fn name() -> Option<String> {
    Some(String::from("Syntax Scoring"))
}

pub fn solve_p1() -> Option<String> {
    unsafe {if SOLUTIONS == (0, 0) {
	let input = include_str!("input.txt");
	SOLUTIONS = get_solution(input);
    }
	return Some(SOLUTIONS.0.to_string());
    }
}

pub fn solve_p2() -> Option<String> {
    unsafe {if SOLUTIONS == (0, 0) {
	let input = include_str!("input.txt");
	SOLUTIONS = get_solution(input);
    }
    	    return Some(SOLUTIONS.1.to_string());
    }
}

static mut SOLUTIONS: (u64, u64) = (0,0);

fn get_solution(input: &str) -> (u64, u64) {
    let mut res = (0, 0);
    let mut incompletes = Vec::new();
    let line_vec = Line::parse_lines(input);
    for l in line_vec {
	match l.get_error() {
	    Error::Corrupt(c) => {
		res.0 += Line::get_corrupt_score(c);
	    },
	    Error::Incomplete(s) => {
		incompletes.push(Line::get_incomplete_score(s));
	    }
	}
    }
    incompletes.sort();
    res.1 = incompletes[(incompletes.len()) /2];
    res
}

enum Typ {
    Opening(char),
    Closing(char),
}

enum Error {
    Corrupt(char),
    Incomplete(String)
}

struct Line<'a> {
    s: &'a str,
}

impl Line<'_> {

    fn parse_line(input: &str) -> Line {
	Line { s: input }
    }

    fn parse_lines(input: &str) -> Vec<Line> {
	let mut res = Vec::new();
	for line in input.lines() {
	    res.push(Line::parse_line(line));
	}
	res
    }

    fn get_error(&self) -> Error {
	let mut vec = Vec::new();
	for c in self.s.chars() {
	    match Line::typ(c) {
		Typ::Opening(_) => vec.push(c),
		Typ::Closing(t) => {
		    if !(vec.pop() == Some(t)) {
			return Error::Corrupt(t);
		    }
		}
	    }
	}
	let mut s = String::new();
	while let Some(c) = vec.pop() {
	    s.push(c);
	}
	Error::Incomplete(s)
	    
    }

    fn typ(c: char) -> Typ {
	match c {
	    '(' => Typ::Opening('('),
	    '[' => Typ::Opening('['),
	    '{' => Typ::Opening('{'),
	    '<' => Typ::Opening('<'),
	    ')' => Typ::Closing('('),
	    ']' => Typ::Closing('['),
	    '}' => Typ::Closing('{'),
	    '>' => Typ::Closing('<'),
	    _ => panic!("`{}` couldn't be typed", c),
	}
    }

    fn get_corrupt_score(c: char) -> u64 {
	match c {
	    '(' => 3,
	    '[' => 57,
	    '{' => 1197,
	    '<' => 25137,
	    _ => panic!(),
	}
    }

    fn get_incomplete_score(s: String) -> u64 {
	let mut res = 0;
	for c in s.chars() {
	    res *= 5;
	    match c {
		'(' => res += 1,
		'[' => res += 2,
		'{' => res += 3,
		'<' => res += 4,
		_ => panic!(),
	    }
	}
	res
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
    fn test_corrupted_char() {
	let line_vec = Line::parse_lines(get_input());
	if let Error::Corrupt(c) = line_vec[2].get_error() {
	    assert_eq!('{', c);
	} else {
	    panic!();
	};
    }

    #[test]
    fn test_invalid_str() {
	let line_vec = Line::parse_lines(get_input());
	if let Error::Incomplete(s) = line_vec[0].get_error() {
	    assert_eq!("{{[[({([", s);
	} else {
	    panic!();
	}
    }
    #[test]
    fn test_part_1() {
	assert_eq!(26397, get_solution(get_input()).0);
    }

    #[test]
    fn test_part_2() {
	assert_eq!(288957, get_solution(get_input()).1);
    }
}
