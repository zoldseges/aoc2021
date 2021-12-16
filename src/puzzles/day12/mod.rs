use std::collections::{HashSet, HashMap};
use crate::utils::graph::Graph;

pub fn name() -> Option<String> {
    Some("Passage Pathing".to_string())
}

pub fn solve_p1() -> Option<String> {
    let input = include_str!("input.txt");
    let g = parse_input(input);
    Some(g.count_paths_p1().to_string())
}

pub fn solve_p2() -> Option<String> {
    None
}

fn parse_input(input: &str) -> Graph {
    let mut graph = Graph::new();
    for line in input.lines() {
	let mut spline = line.split('-');
	graph.insert_conn(spline.next().unwrap(), spline.next().unwrap());
    }
    graph
}

impl<'a> Graph<'a> {

    pub fn count_paths_p1(&self) -> u32 {
	self.traverse_p1("start", HashSet::new())
    }

    // travereses through every path, returns 1 when it gets to "end"
    fn traverse_p1(&self, pos: &'a str, mut visited: HashSet<&'a str>) -> u32 {
	let mut cont = 0;

	if pos == "end" {
	    return 1;
	}
	if let Some(t) = pos.chars().next() {
	    if t.is_ascii_lowercase() {
		visited.insert(pos);
	    }
	}
	for id in self.get_adjs(pos).unwrap() - &visited {
	    cont += self.traverse_p1(id, visited.clone());
	}
	cont
    }

    pub fn count_paths_p2(&self) -> u32 {
	let mut visited = HashMap::new();
	visited.insert("start", 2);
	self.traverse_p2("start", visited)
    }

    fn traverse_p2(&self, pos: &'a str, mut visited: HashMap<&'a str, u8>) -> u32 {
	let mut cont = 0;
	if pos == "end" {
	    return 1;
	}
	
	if let Some(t) = pos.chars().next() {
	    if t.is_ascii_lowercase() {
		let c = visited.entry(pos).or_insert(0);
		*c += 1;
	    }
	}
	let mut legal_adjs = HashSet::new();
	let mut flag = true;
	for id in self.get_adjs(pos).unwrap() {
	    if let Some(t) = visited.get(id) {
		if t == &1 && flag {
		    legal_adjs.insert(id);
		    flag = false;
		}
	    } else {
		legal_adjs.insert(id);
	    }
	}
	for id in legal_adjs {
	    cont += self.traverse_p2(id, visited.clone());
	}
	cont
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    fn get_input_0() -> &'static str {
	include_str!("input_test_0.txt")
    }

    fn get_input_1() -> &'static str {
	include_str!("input_test_1.txt")
    }

    fn get_input_2() -> &'static str {
	include_str!("input_test_2.txt")
    }

    #[test]
    fn count_paths_p1() {
	let g = parse_input(get_input_0());
	assert_eq!(10, g.count_paths_p1());
	let g = parse_input(get_input_1());
	assert_eq!(19, g.count_paths_p1());
	let g = parse_input(get_input_2());
	assert_eq!(226, g.count_paths_p1());
    }

    #[test]
    fn count_paths_p2() {
	let g = parse_input(get_input_0());
	assert_eq!(36, g.count_paths_p2());
	let g = parse_input(get_input_1());
	assert_eq!(103, g.count_paths_p2());
	let g = parse_input(get_input_2());
	assert_eq!(3059, g.count_paths_p2());
    }
}
