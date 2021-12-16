use std::collections::HashSet;
use std::ops::{Add, Sub};

pub fn name() -> Option<String> {
    Some(String::from("Hydrothermal Venture"))
}

pub fn solve_p1() -> Option<String> {
    let input = include_str!("input.txt");
    Some(String::from(format!("{}",
			      get_solution(true, input))))
}

pub fn solve_p2() -> Option<String> {
    let input = include_str!("input.txt");
    Some(String::from(format!("{}",
			      get_solution(false, input))))
}

#[derive (Debug, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) ->  Self {
	Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) ->  Self {
	Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive (Debug, PartialEq, Clone, Copy)]
struct Line {
    p0: Point,
    p1: Point,
    dir: Point,
    curr_intermediate: Point,
}

impl Line {
    fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Line {
	let (mut x_dir, mut y_dir) = (x1-x0, y1-y0);
	if x_dir != 0 {
	    x_dir = x_dir / x_dir.abs();
	}
	if y_dir != 0 {
	    y_dir = y_dir / y_dir.abs();
	}
	Line {
	    p0: Point {x: x0, y: y0},
	    p1: Point {x: x1, y: y1},
	    dir: Point {x :x_dir, y: y_dir},
	    curr_intermediate: Point {x: x0, y: y0},
	}
    }
}

impl Iterator for Line {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
	let ret = Some(self.curr_intermediate);
	if self.curr_intermediate - self.dir == self.p1 {
	    None
	} else {
	    self.curr_intermediate = self.curr_intermediate + self.dir;
	    ret
	}
    }
}

fn str_to_line(s: &str) -> Line {
    let sp: Vec<&str> = s.split(
	|x| { x == ',' || x == '-' || x == '>'}).
	collect();
    Line::new(sp[0].trim().parse().unwrap(),
	      sp[1].trim().parse().unwrap(),
	      sp[sp.len()-2].trim().parse().unwrap(),
	      sp[sp.len()-1].trim().parse().unwrap(),
    )
}

fn is_right_line(l: &Line) -> bool {
    l.p0.y == l.p1.y || l.p0.x == l.p1.x
}

fn draw_line(line: &Line, map: &mut HashSet<(i32, i32)>, danger_map: &mut HashSet<(i32, i32)>) {
    for p in line.into_iter() {
	match map.get(&(p.x, p.y)) {
	    Some(_) => {
		danger_map.insert((p.x, p.y));
	    },
	    None => {map.insert((p.x, p.y));},
	};
    }
}

fn get_solution(only_right: bool, input: &str) -> usize {
    let mut map = HashSet::new();
    let mut danger_map = HashSet::new();
    for line in input.lines() {
	let line = str_to_line(line);
	if only_right {
	    if is_right_line(&line) {
		draw_line(&line, &mut map, &mut danger_map);
	    }
	} else {
	    draw_line(&line, &mut map, &mut danger_map);
	}
    }
    danger_map.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
	include_str!("input_test.txt")
    }

    #[test]
    fn test_str_to_line() {
	let s = "15,2 -> 8,9";
	let l = Line::new(15, 2, 8, 9);
	assert_eq!(l, str_to_line(s));
    }

    #[test]
    fn test_part1() {
	assert_eq!(5, get_solution(true, get_input()));
    }
    
    #[test]
    fn test_part2() {
	assert_eq!(12, get_solution(false, get_input()));
    }
}

