//! # Seven segment digits
//!
//! ## Digit representation
//! ```text
//!   0:      1:      2:      3:      4:  
//!  AAAA    ....    AAAA    AAAA    .... 
//! B    C  .    C  .    C  .    C  B    C
//! B    C  .    C  .    C  .    C  B    C
//!  ....    ....    DDDD    DDDD    DDDD 
//! E    F  .    F  E    .  .    F  .    F
//! E    F  .    F  E    .  .    F  .    F
//!  GGGG    ....    GGGG    GGGG    .... 
//!                                       
//!   5:      6:      7:      8:      9:  
//!  AAAA    AAAA    AAAA    AAAA    AAAA 
//! B    .  B    .  .    C  B    C  B    C
//! B    .  B    .  .    C  B    C  B    C
//!  DDDD    DDDD    ....    DDDD    DDDD 
//! .    F  E    F  .    F  E    F  .    F
//! .    F  E    F  .    F  E    F  .    F
//!  GGGG    GGGG    ....    GGGG    GGGG 
//! ```
//! As the letters represent segments,
//! we count their occurences and also
//! get the length of the word it was
//! seen in. From this two information
//! of all words we can deduce what
//! segment a letter represents.
//!
//! Numbers can be represented in binary
//! by the segments they include
//! ```text
//! |     | A | B | C | D | E | F | G | wlen |
//! |-----+---+---+---+---+---+---+---+------|
//! |   0 | x | x | x |   | x | x | x |    6 |
//! |   1 |   |   | x |   |   | x |   |    2 |
//! |   2 | x |   | x | x | x |   | x |    5 |
//! |   3 | x |   | x | x |   | x | x |    5 |
//! |   4 |   | x | x | x |   | x |   |    4 |
//! |   5 | x | x |   | x |   | x | x |    5 |
//! |   6 | x | x |   | x | x | x | x |    6 |
//! |   7 | x |   | x |   |   | x |   |    3 |
//! |   8 | x | x | x | x | x | x | x |    7 |
//! |   9 | x | x | x | x |   | x | x |    6 |
//! |-----+---+---+---+---+---+---+---+------|
//! | occ | 8 | 6 | 8 | 7 | 4 | 9 | 7 |      |
//! ```
//! x marks if the bit on this place is on or off, 
//! so 0 would be 0b01110111,
//! 1 would be 0b00010010 and so on...

pub fn name() -> Option<String> {
    Some(String::from("Seven Segment Search"))
}

pub fn solve_p1() -> Option<String> {
    let input = include_str!("input.txt");
    Some(String::from(format!(
	"{}", get_solution_p1(input))))
}

pub fn solve_p2() -> Option<String> {
    let input = include_str!("input.txt");
    Some(String::from(format!(
	"{}", get_solution_p2(input))))
}

fn get_solution_p1(input: &str) -> u32 {
    let vec_of_lines = parse_input(input);
    let mut acc = 0;
    for line in vec_of_lines {
	for word in line.get_out() {
	    let len = word.len();
	    if len == 2 || len == 3 ||
		len == 4 || len == 7 {
		    acc += 1;
		}
	}
    }
    acc
}

struct Line<'a> {
    clues: [&'a str; 10],
    out: [&'a str; 4],
}

impl<'a> Line<'a> {
    fn get_clues(&self) -> &[&'a str;10] {
	&self.clues
    }

    fn get_out(&self) -> &[&'a str;4] {
	&self.out
    }

}

fn parse_input(input: &str) -> Vec<Line> {
    let mut res = Vec::new();
    for line in input.lines() {
	res.push(parse_line(line));
    }
    res
}

fn parse_line(input: &str) -> Line {
    let mut split = input.split('|');
    let clues = split.next().unwrap();
    let out = split.next().unwrap();
    Line {
	clues: clues.split_whitespace().
	    collect::<Vec<&str>>().try_into().unwrap(),
	out: out.split_whitespace().
	    collect::<Vec<&str>>().try_into().unwrap(),
    }
}

/// # Make translational array
///
/// Creates an array like this:
/// 
/// |  a  |  b  |  c        | ... |  g  |
/// |-----|-----|-----------|-----|-----|
/// | ... | ... | 0b0000001 | ... | ... |
///
/// Which means that the letter c in words shoud
/// turn the last segment on (originally labelled as G)
fn mk_trans_array(line: &Line) -> [u8;7] {
    let seg_keys = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let mut trans_arr: [u8; 7] = [0; 7];
    // theese words have unique lengths, we need them
    // to determine segments A,C,D,G as their occurence
    // matches
    // if a key occurs 8 times and is in the word for 4,
    // then the key represents segment A, else C
    // similarly if a key occurs 7 times and in the word
    // for 4, it's D else, it's G
    let mut word_num4: &str = "";
    for word in line.get_clues() {
	if word.len() == 4 {
	    word_num4 = word;
	}
    }
    let word_num4 = word_num4;
    for (i, seg_key) in seg_keys.iter().enumerate() {
	let mut occ: u8 = 0;
	for word in line.get_clues() {
	    if word.chars().any(|x| { x == *seg_key }) {
		occ += 1;
	    }
	}
	trans_arr[i] = det_seg(occ, *seg_key, word_num4);
    }
    trans_arr
}

fn det_seg(occ: u8,
	   seg_key: char,
	   word_num4: &str)
	   -> u8 {
    let in_word_num_4 = word_num4.chars().
	any(|x| { x == seg_key });
    match occ {
	//               ABC DEFG
	4 =>         0b0000_0100,
	6 =>         0b0010_0000,
	9 =>         0b0000_0010,
	8 => match in_word_num_4 {
	    true  => 0b0001_0000,
	    false => 0b0100_0000,
	}
	7 => match in_word_num_4 {	
	    true  => 0b0000_1000,
	    false => 0b0000_0001,
	}
	_ => panic!(),
    }
}

fn word_to_digit(word: &str, trans_arr: [u8;7]) -> u8 {
    let mut res = 0;
    for c in word.chars() {
	match c {
	    'a' => res |= trans_arr[0],
	    'b' => res |= trans_arr[1],
	    'c' => res |= trans_arr[2],
	    'd' => res |= trans_arr[3],
	    'e' => res |= trans_arr[4],
	    'f' => res |= trans_arr[5],
	    'g' => res |= trans_arr[6],
	    _ => panic!(),
	}
    }
    res
}

fn vec_to_dec(inp: [u8;4]) -> u32 {
    let mut res = 0;
    for n in inp {
	res *= 10;
	res += n as u32
    }
    res
}

// converts digit represented by its segment
// to decimal digit
fn seg_to_dec(seg: u8) -> u8 {
    match seg {
	0b0111_0111 => 0,
	0b0001_0010 => 1,	
	0b0101_1101 => 2,
	0b0101_1011 => 3,
	0b0011_1010 => 4,
	0b0110_1011 => 5,
	0b0110_1111 => 6,
	0b0101_0010 => 7,
	0b0111_1111 => 8,	
	0b0111_1011 => 9,
	_ => panic!(),
    }
}

fn trans_line(line: &Line) -> u32 {
    let trans_array = mk_trans_array(line);
    let mut res_vec = [0, 0, 0, 0];
    for (i, word) in line.get_out().iter().enumerate() {
	res_vec[i] = seg_to_dec(
	    word_to_digit(word, trans_array));
    }
    vec_to_dec(res_vec)
}

fn get_solution_p2(input: &str) -> u32 {
    let vec_of_lines = parse_input(input);
    let mut acc = 0;
    for line in vec_of_lines {
	acc += trans_line(&line);
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn get_input() -> &'static str {
	include_str!("input_test.txt")
    }

    fn get_oneline_input() -> &'static str {
	"acedgfb cdfbe gcdfa fbcad dab cefabd \
	 cdfgeb eafb cagedb ab | \
	 cdfeb fcadb cdfeb cdbaf"
    }

    #[test]
    fn test_part_1() {
	assert_eq!(26, get_solution_p1(get_input()));
    }

    #[test]
    fn test_mk_trans_arr() {
	let vec = parse_input(get_oneline_input());
	let trans_arr = mk_trans_array(&vec[0]);
	//             ABC DEFG
	assert_eq!([0b0001_0000,
		    0b0000_0010,
		    0b0000_0001,
		    0b0100_0000,
		    0b0010_0000,
		    0b0000_1000,
		    0b0000_0100,],
		   trans_arr);

    }

    #[test]
    fn test_vec_to_dec() {
	assert_eq!(9485, vec_to_dec([9, 4, 8, 5]));
    }

    #[test]
    fn trans_one_line() {
	let vec = parse_input(get_oneline_input());
	assert_eq!(5353, trans_line(&vec[0]));
    }

    #[test]
    fn test_part_2() {
	assert_eq!(61229, get_solution_p2(get_input()));
    }
}
