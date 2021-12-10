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

pub fn parse_input(input: &str) -> Vec<u32> {
    let res = input.split(',').
	map(|x| { x.trim() }).
	map(|x| { x.parse::<u32>().unwrap()}).
	collect();
    res
}

pub fn _steping_function(input: u32) -> u32 {
    if input == 0 {
	return 0;
    } else {
	return input + steping_function(input - 1);
    }
}

pub fn steping_function(input: u32) -> u32 {
    (input + 1) * input / 2
}

pub fn calc_min_pos(linear: bool, positions: Vec<u32>) -> (u32, u32) {
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
