//! # Day 8
//! ## Concepts
//! Numbers are represented as
//! by their segments as the following
//! table shows
//!
//! |         | ABCDEFG |   |
//! |---------|---------|---|
//! | ABCEFG  | 1110111 | 0 |
//! | C F     | 0010010 | 1 |
//! | ACDE G  | 1011011 | 2 |
//! | ACDFG   | 1011011 | 3 |
//! | BCDF    | 0111010 | 4 |
//! | ABDFG   | 1101011 | 5 |
//! | ABDEFG  | 1101111 | 6 |
//! | ACF     | 1010010 | 7 |
//! | ABCDEFG | 1111111 | 8 |
//! | ABCDFG  | 1111011 | 9 |

pub fn name() -> Option<String> {
    Some(String::from("Seven Segment Search"))
}

pub fn solve_p1() -> Option<String> {
    let input = include_str!("input.txt");
    Some(String::from(format!(
	"{}", get_p1(parse_input(input)))))
}

pub fn solve_p2() -> Option<String> {
    None
}

/// clue the left side of the input\
/// out the right side (from which the solution comes) of the input
#[derive (Debug)]
pub struct Line<'a> {
    pub clue : Vec<&'a str>,
    pub out  : Vec<&'a str>,
}

pub fn parse_input(input: &str) -> Vec<Line> {
    let mut res = Vec::new();
    for line in input.lines() {
	let mut p_line = Line { clue: Vec::new(),
				out: Vec::new(),};
	let mut line_split = line.split('|');
	p_line.clue = line_split.next().unwrap().
	    split_whitespace().
	    map( | x | { x.trim() } ).
	    collect();
	p_line.out = line_split.next().unwrap().
	    split_whitespace().
	    map( | x | { x.trim() } ).
	    collect();
	res.push(p_line);
    }
    res
}

pub fn get_p1(input: Vec<Line> ) -> u32 {
    let mut result = 0;
    for v in input {
	for s in v.out {
	    let len = s.len();
	    if len == 2 || len == 3 ||
		len == 4 || len == 7 {
		    result += 1;
		}
	}
    }
    result
}

/// # Extracts the "translation array" from Line.clues
/// The resulting array looks like this:\
/// [a, b, c, d, e, f, g]\
/// where each of them are a binary number
/// representing the possible segments a letter
/// represents. For more information about the
/// representation check the crate
/// documentation [here](crate::puzzles::day08)
fn mk_trans_arr(line: &Line) -> [Segment; 7] {
    let chars = ['a', 'b', 'c', 'd',
		 'e', 'f', 'g'];
    let mut seg_arr: [Segment; 7] = [Segment::new(),
				     Segment::new(),
				     Segment::new(),
				     Segment::new(),
				     Segment::new(),
				     Segment::new(),
				     Segment::new(),
    ];
    for (i,c) in chars.iter().enumerate() {
	let count = count_occurence(line, *c);
	let poss_segs = poss_seg_from_occurence(count);
	seg_arr[i].c = *c;
	seg_arr[i].poss_segs = poss_segs;
    }
    seg_arr
}

/// poss_segs is the possible segments [Segment.c]
/// represents in binary.
struct Segment {
    c: char,
    pub poss_segs: u8,
}

impl Segment {
    fn new() -> Segment {
	Segment { c: '0', poss_segs: 0, }
    }
}

pub fn count_occurence(input: &Line, c: char) -> u8 {
    let mut counter = 0;
    let valid_counts = [4,6,7,8,9];
    for s in &input.clue {
	if s.chars().any(|x| { x == c}) {
	    counter += 1;
	}
    }
    if valid_counts.iter().any(|&x| { x == counter }) {
	counter
    } else {
	panic!("invlaid number of occurence of `{}` \
		in {:?}", c, input);
    }
}

fn poss_seg_from_occurence(occ: u8) -> u8 {
    match occ {
	//   0b0ABC_DEFG
	4 => 0b0000_0100,
	6 => 0b0010_0000,
	7 => 0b0000_1001,
	8 => 0b0101_0000,
	9 => 0b0000_0010,
	_ => panic!("Impossibru!"),
    }
}

const NUMS: [u8; 10] = [0b0111_0111,
			0b0001_0010,
			0b0101_1101,
			0b0101_1011,
			0b0011_1010,
			0b0110_1011,
			0b0110_1111,
			0b0101_0010,
			0b0111_1111,
			0b0111_1011,];

pub fn translate_line(line: &Line) -> u32 {
    let mut res_arr = [0_u8; 4];
    let trans_arr = mk_trans_arr(&line);
    for (i, word) in line.out.iter().enumerate() {
	for c in word.chars() {
	    let poss_segs = letter_to_seg_i(c);
	    let poss_segs = trans_arr[poss_segs].
		poss_segs;
	    // this has to be unmasked later
	    res_arr[i] |= poss_segs;
	}
	for (j, num) in NUMS.iter().enumerate() {
	    // let cand = res_arr[i] & num;
	    // if cand == *num {
	    // 	res_arr[i] = j as u8;
	    // }

	    // if NUMS.iter().any(|x| { cand == *x } ) {
	    // 	println!("{}", cand);
	    // 	res_arr[i] = j as u8;
	    // }
	}
    }
    arr_to_dec(res_arr)
}

pub fn temp_debug_printer(line: &Line) {
    let mut res_arr = [0_u8; 10];
    let trans_arr = mk_trans_arr(&line);
    let sols = [8,5,2,3,7,9,6,4,0,1];
    for (i, word) in line.clue.iter().enumerate() {
	for c in word.chars() {
	    let poss_segs = letter_to_seg_i(c);
	    let poss_segs = trans_arr[poss_segs].
		poss_segs;
	    // this has to be unmasked later
	    res_arr[i] |= poss_segs;
	}
	println!("{:>8} -> {:08b}", word, res_arr[i]);
	println!("{:>8} -> {:08b}", sols[i], NUMS[sols[i]]);
	println!();
	for (j, num) in NUMS.iter().enumerate() {
	    let rem = num ^ res_arr[i];
	    println!("{:>8} ^= {:08b}", j, rem);
	}
	println!("{:->20}", "");
    }
}

pub fn arr_to_dec(arr: [u8;4]) -> u32 {
    let mut res: u32 = 0;
    for n in arr {
	res *= 10;
	res += n as u32;
    }
    res
}

/// converts number to the index in the pos_seg_arr
fn letter_to_seg_i(c: char) -> usize {
    match c {
	'a' => 0,
	'b' => 1,
	'c' => 2,
	'd' => 3,
	'e' => 4,
	'f' => 5,
	'g' => 6,
	_   => panic!(),
    }
}
