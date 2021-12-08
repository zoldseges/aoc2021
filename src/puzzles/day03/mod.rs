pub fn name() -> Option<String> {
    Some(String::from("Binary Diagnostic"))
}

pub fn solve_p1() -> Option<String> {
    let input = include_str!("input.txt");
    Some(get_solution_p1(input))
}

pub fn solve_p2() -> Option<String> {
    None
}

pub fn get_solution_p1(input: &str) -> String {
    let mut counts = Vec::new();
    let mut line_count = 0;
    let mut gamma_vec = Vec::new();
    let mut epsilon_vec = Vec::new();
    let gamma;
    let epsilon;

    let mut lines = input.lines();
    for chr in lines.next().unwrap().chars() {
	counts.push(chr.to_digit(10).unwrap());
	gamma_vec.push(0);
	epsilon_vec.push(0);
    }
    line_count += 1;
    for line in lines {
	for (i, chr) in line.chars().enumerate() {
	    if chr == '1' {
		counts[i] += 1;
	    }
	}
	line_count += 1;
    }

    for i in 0..counts.len() {
	if counts[i] >= line_count / 2 {
	    gamma_vec[i] = 1;
	} else {
	    epsilon_vec[i] = 1;
	}
    }
    gamma = bin_vec_to_dec(gamma_vec);
    epsilon = bin_vec_to_dec(epsilon_vec);
    format!("{}", gamma * epsilon)
}

pub fn get_part_p2(input: &str, oxy: bool) -> i32 {
    let mut node = &mut Node::new();
    let mut res_string = String::new();
    let mut cmp: i32;
    for line in input.lines() {
	node.add_str_to_tree(line);
    }
    loop {
	if node.get_side_node(0).get_count() == 0 &&
	    node.get_side_node(1).get_count() == 0 {
		break;
	    }
	cmp = node.get_side_node(0).get_count() -
	    node.get_side_node(1).get_count();
	if oxy {
	    match cmp {
		i32::MIN..=-1 => {
		    node = node.get_side_node(1);
		    res_string += "1";
		}
		1..=i32::MAX => {
		    node = node.get_side_node(0);
		    res_string += "0";
		}
		0 => {
		    node = node.get_side_node(1);
		    res_string += "1";
		}
	    }
	} else {
	    match cmp {
		i32::MIN..=-1 => {
		    node = node.get_side_node(0);
		    res_string += "0";
		}
		1..=i32::MAX=> {
		    node = node.get_side_node(1);
		    res_string += "1";
		}
		0  => {
		    node = node.get_side_node(1);
		    res_string += "0";
		}
	    }
	}
    }
    println!("{}", res_string);
    i32::from_str_radix(&res_string, 2).
	expect(&(format!("couldn't convert {} to decimal", &res_string)))
}

pub struct Node {
    count: i32,
    left_0: Box<Option<Node>>,
    right_1: Box<Option<Node>>,
}

impl Node {
    pub fn new()-> Node {
	Node { count: 0,
	       left_0: Box::new(None),
	       right_1: Box::new(None)
	}
    }

    pub fn add_node(&mut self, side: i8) {
	let new = Node::new();
	match side {
	    0 => self.left_0 = Box::new(Some(new)),
	    1 => self.right_1 = Box::new(Some(new)),
	    _ => panic!("bad side parameter `{}`", side)
	}
    }

    pub fn get_side_node(&mut self, side: i8) -> &mut Node {
	match side {
	    0 => match self.left_0.as_ref().as_ref() {
		Some(_) => self.left_0.as_mut().as_mut().unwrap(),
		None => {
		    self.add_node(0);
		    self.left_0.as_mut().as_mut().unwrap()
		}
	    },
	    1 => match self.right_1.as_ref().as_ref() {
		Some(_) => self.right_1.as_mut().as_mut().unwrap(),
		None => {
		    self.add_node(1);
		    self.right_1.as_mut().as_mut().unwrap()
		}
	    }
	    _ => panic!("bad side parameter `{}`", side),
	}
    }

    pub fn add_str_to_tree(&mut self, s: &str) {
	let mut side: i8;
	let mut curr_node = self;
	curr_node.inc_count();
	for chr in s.chars() {
	    side = chr.to_digit(10).unwrap().try_into().unwrap();
	    curr_node = curr_node.get_side_node(side);
	    curr_node.inc_count();
	}
    }

    pub fn inc_count(&mut self) {
	self.count += 1
    }

    pub fn get_count(&self) -> i32 {
	self.count
    }
}

pub fn bin_vec_to_dec(mut bin: Vec<i32>) -> i32 {
    let mut ret_val = 0;
    bin.reverse();
    for (i, value) in bin.iter().enumerate() {
	if value.to_owned() == 1 {
	    ret_val += 2_i32.pow(i.try_into().unwrap());
	}
    }
    ret_val
}
