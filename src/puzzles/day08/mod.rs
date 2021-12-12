//! # Seven segment digits
//!
//! ## Digit representation
//! ```ignore
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
//! read the length of the word it was
//! seen in. From this two information
//! of all words we can deduce what
//! segment a letter represents.
//!
//! Numbers can be represented in binary
//! by the segments they include
//! ```ignore
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
    None
}

pub fn solve_p2() -> Option<String> {
    None
}

pub struct Line<'a> {
    clues: [&'a str; 10],
    out: [&'a str; 4],
}

impl<'a> Line<'a> {
    pub fn get_clues(&self) -> &[&'a str;10] {
	&self.clues
    }

    pub fn get_out(&self) -> &[&'a str;4] {
	&self.out
    }

}

pub fn read_input(input: &str) -> Vec<Line> {
    let mut res = Vec::new();
    for line in input.lines() {
	res.push(read_line(line));
    }
    res
}

pub fn read_line(input: &str) -> Line {
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

#[cfg(test)]
mod tests {
    use super::*;
    
    fn get_oneline_input() -> &'static str {
	"acedgfb cdfbe gcdfa fbcad dab cefabd \
	 cdfgeb eafb cagedb ab | \
	 cdfeb fcadb cdfeb cdbaf"
    }

    #[test]
    fn test_mk_trans_arr() {
	let vec = read_input(get_oneline_input());
	let trans_arr = mk_trans_array(&vec[0]);
	panic!();
    }
}
