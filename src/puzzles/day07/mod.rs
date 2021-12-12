pub fn name() -> Option<String> {
    Some(String::from("The Treachery of Whales"))
}

pub fn solve_p1() -> Option<String> {
    let input = include_str!("input.txt");
    let input = parse_input(input);
    let pos = calc_min_pos(true, input);
    Some(String::from(format!(
	"{}", pos.0)))
}

pub fn solve_p2() -> Option<String> {
    let input = include_str!("input.txt");
    let input = parse_input(input);
    let pos = calc_min_pos(false, input);
    Some(String::from(format!(
	"{}", pos.0)))
}

fn parse_input(input: &str) -> Vec<u32> {
    let res = input.split(',').
	map(|x| { x.trim() }).
	map(|x| { x.parse::<u32>().unwrap()}).
	collect();
    res
}

fn _steping_function(input: u32) -> u32 {
    if input == 0 {
	return 0;
    } else {
	return input + steping_function(input - 1);
    }
}

fn steping_function(input: u32) -> u32 {
    (input + 1) * input / 2
}

fn calc_min_pos(linear: bool, positions: Vec<u32>) -> (u32, u32) {
    let mut min = (u32::MAX, 0);
    let mut tmp: (u32, u32);
    for i in 0..=*positions.iter().max().unwrap() {
	tmp = (0, i);
	for pos in &positions {
	    if linear {
		tmp.0 += (i as i32 - *pos as i32).abs() as u32;
	    } else {
		tmp.0 += steping_function((i as i32 - *pos as i32).abs() as u32);
	    }
	}
	if tmp.0 < min.0 {
	    min = tmp;
	}
    }
    min
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
	include_str!("input_test.txt")
    }

    #[test]
    fn test_parser() {
	assert_eq!(vec!(16,1,2,0,4,2,7,1,2,14), parse_input(get_input()));
    }

    #[test]
    fn test_calc_min_lin_pos() {
	assert_eq!((37, 2), calc_min_pos(true, parse_input(get_input())));
    }

    #[test]
    fn test_step_func() {
	assert_eq!(66, steping_function((5 as i32 - 16 as i32).abs() as u32));
    }

    #[test]
    fn test_calc_min_step_pos() {
	assert_eq!((168, 5), calc_min_pos(false, parse_input(get_input())));
    }
}
