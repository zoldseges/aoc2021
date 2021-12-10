pub fn name() -> Option<String> {
    Some(String::from("Lanternfish"))
}

pub fn solve_p1() -> Option<String> {
    let input = include_str!("input.txt");
    let fishes = parse_input(input);
    Some(String::from(format!(
	"{}", no_fishes_after_n_days(&fishes, 80))))
}

pub fn solve_p2() -> Option<String> {
    let input = include_str!("input.txt");
    let fishes = parse_input(input);
    Some(String::from(format!(
	"{}", no_fishes_after_n_days(&fishes, 256))))
}

pub fn parse_input(input: &str) -> Vec<u8> {
    let mut res = Vec::new();
    for n in input.split(',') {
	res.push(n.trim().parse::<u8>().unwrap());
    }
    res
}

pub fn no_fishes_after_n_days(fishes: &Vec<u8>, no_days: u32) -> u64 {
    let mut fishes_by_day = [0;9];
    for n in fishes {
	fishes_by_day[*n as usize] += 1;
    }
    for _day in 0..no_days {
	let newborns = fishes_by_day[0];
	for bucket in 0..8 {
	    fishes_by_day[bucket] = fishes_by_day[bucket+1]
	}
	fishes_by_day[8] = newborns;
	fishes_by_day[6] += newborns;
    }
    fishes_by_day.iter().sum()
}
