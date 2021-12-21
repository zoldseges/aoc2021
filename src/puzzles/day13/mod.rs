pub fn name() -> Option<String> {
    Some("Transparent Origami".to_string())
}

pub fn solve_p1() -> Option<String> {
    None
}

pub fn solve_p2() -> Option<String> {
    None
}

#[derive(Debug)]
struct Input {
    coords: Vec<(u8, u8)>,
    folds: Vec<(u8, u8)>,
}

fn parse_input(input: &str) -> Input {
    let mut coords = Vec::new();
    let mut folds = Vec::new();
    for line in input.lines() {
	if let Some(t) = line.find(",") {
	    let spline = line.split(",");
	    let mut coord = spline.collect::<Vec<&str>>();
	    let y = coord.pop().unwrap().parse::<u8>().unwrap();	
	    let x = coord.pop().unwrap().parse::<u8>().unwrap();
	    coords.push((x, y));
	} else if let Some(t) = line.find("=") {
	    let mut spline = line.split("=");
	    spline.next();
	    let mut x = 0;
	    let mut y = 0;
	    if let Some(t) = line.find("x") {
		x = spline.next().unwrap().parse::<u8>().unwrap();
	    } else {
		y = spline.next().unwrap().parse::<u8>().unwrap();
	    }
	    folds.push((x, y));
	}
    }
    Input { coords, folds }
}

impl Input {
    fn fold(&mut self) {

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
    fn test_parse() {
	println!("{:?}", parse_input(get_input()));
	panic!();
    }
}
