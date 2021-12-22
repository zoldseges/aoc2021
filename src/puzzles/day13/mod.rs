use std::cmp::Ordering;
use std::collections::HashSet;

pub fn name() -> Option<String> {
    Some("Transparent Origami".to_string())
}

pub fn solve_p1() -> Option<String> {
    let input = include_str!("input.txt");
    Some(get_solution_p1(input).to_string())
}

pub fn solve_p2() -> Option<String> {
    let input = include_str!("input.txt");
    let sol = "\n\t\t".to_string() + &get_solution_p2(input).replace("\n", "\n\t\t");
    Some(sol)
}

#[derive(Debug)]
struct Input {
    coords: HashSet<(u32, u32)>,
    folds: Vec<(u32, u32)>,
}

fn parse_input(input: &str) -> Input {
    let mut coords = HashSet::new();
    let mut folds = Vec::new();
    for line in input.lines() {
	if let Some(_) = line.find(",") {
	    let spline = line.split(",");
	    let mut coord = spline.collect::<Vec<&str>>();
	    let y = coord.pop().unwrap().parse::<u32>().unwrap();	
	    let x = coord.pop().unwrap().parse::<u32>().unwrap();
	    coords.insert((x, y));
	} else if let Some(_) = line.find("=") {
	    let mut spline = line.split("=");
	    spline.next();
	    let mut x = 0;
	    let mut y = 0;
	    if let Some(_) = line.find("x") {
		x = spline.next().unwrap().parse::<u32>().unwrap();
	    } else {
		y = spline.next().unwrap().parse::<u32>().unwrap();
	    }
	    folds.push((x, y));
	}
    }
    folds.reverse();
    Input { coords, folds }
}

impl Input {

    fn fold(&mut self) -> bool {
	let fold = self.folds.pop();
	let old_cords = self.coords.clone();
	match fold {
	    Some((fold_x, 0)) => {
		for (x, y) in old_cords {
		    if x > fold_x {
			let new_x = 2 * fold_x - x;
			self.coords.remove(&(x,y));
			self.coords.insert((new_x, y));
		    }
		}
		true
	    },
	    Some((0, fold_y)) => {
		for (x, y) in old_cords {
		    if y > fold_y {
			let new_y = 2 * fold_y - y;
			self.coords.remove(&(x,y));
			self.coords.insert((x, new_y));
		    }
		}
		true
	    },
	    _ => false,
	}
    }

    fn get_coord_count(&self) -> usize {
	self.coords.len()
    }

    fn get_sorted_coords(&self) -> Vec<(u32, u32)> {
	let mut vec = Vec::new();
	for coord in &self.coords {
	    vec.push(*coord);
	}
	vec.sort_by( |a, b| {
	    if a.1 > b.1 { return Ordering::Greater; }
	    if a.1 < b.1 { return Ordering::Less; }

	    if a.0 > b.0 { return Ordering::Greater; }
	    if a.0 < b.0 { return Ordering::Less; }
	    return Ordering::Equal;
	});
	vec
    }

    fn get_string(&self) -> String {
	let mut coords = self.get_sorted_coords();
	coords.reverse();
	let mut line = 0;
	let mut col = 0;
	let mut string = String::new();
	while let Some(cord) = coords.pop() {
	    while cord.1 > line {
		string += "\n";
		col = 0;
		line += 1;
	    }
	    while cord.0 != col {
		string += " ";
		col += 1;
	    }
	    string += "#";
	    col += 1;
	}
	string
    }
}

fn get_solution_p1(input: &str) -> usize {
    let mut input = parse_input(input);
    input.fold();
    input.get_coord_count()
}

fn get_solution_p2(input: &str) -> String {
    let mut input = parse_input(input);
    while input.fold() {}
    input.get_string()
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
	assert_eq!(17, get_solution_p1(get_input()));
    }
    
    #[test]
    fn test_part_2() {
	let s = "#####\n\
		 #   #\n\
		 #   #\n\
		 #   #\n\
		 #####";
	let s = s.to_string();
	println!("{}", get_solution_p2(get_input()));
	assert_eq!(s, get_solution_p2(get_input()));
    }
}
