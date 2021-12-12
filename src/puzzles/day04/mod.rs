use std::collections::{HashMap, HashSet};

pub fn name() -> Option<String> {
    Some(String::from("Giant Squid"))
}

pub fn solve_p1() -> Option<String> {
    let input = include_str!("input.txt");
    let (sum, draw) = run(true, read_input(input));
    Some(String::from(format!("{}", sum * draw)))
}

pub fn solve_p2() -> Option<String> {
    let input = include_str!("input.txt");
    let (sum, draw) = run(false, read_input(input));
    Some(String::from(format!("{}", sum * draw)))
}

fn read_input(input: &str) ->
    (Vec<i32>, Vec<[[i32; 5]; 5]>, HashMap<i32, Vec<(usize, usize, usize)>>)
{
    let mut hashmap = HashMap::new();
    let mut draws = Vec::new();
    let mut boards = Vec::new();
    let mut board = [[0; 5]; 5];
    let mut lines = input.lines();
    for n in lines.next().unwrap().split(',') {
	draws.push(n.parse::<i32>().unwrap());
    }
    while lines.next().is_some() {
	let board_no = boards.len();
	for i in 0..5 {
	    for (j, n) in lines.next().unwrap().split_whitespace().enumerate() {
		let n = n.parse::<i32>().unwrap();
		board[i][j] = n;
		let vec = hashmap.entry(n).or_insert(Vec::new());
		vec.push((board_no, i, j));
	    }
	}
	boards.push(board);
    }
    (draws, boards, hashmap)
}

fn check_win(board: [[i32; 5]; 5],
	     (_, y, x): (usize, usize, usize)) -> bool
{
    let mut row = true;
    let mut col = true;
    for i in 0..5 {
	if board[i][x] != -1 {
	    row = false;
	    break;
	}
    }
    for i in 0..5 {
	if board[y][i] != -1 {
	    col = false;
	    break;
	}
    }
    row || col
}

fn run(first_winning: bool,
	   (draws, mut boards, hashmap):
	   (Vec<i32>,
	    Vec<[[i32; 5]; 5]>,
	    HashMap<i32, Vec<(usize, usize, usize)>>)) ->
    (i32, i32)
    // returns (sum, last draw)
{
    let mut sum  = -1;
    let mut winning_boards: HashSet<usize> = HashSet::new();
    let mut l_draw = -1;
    let mut win_set_len = 1;
    if !first_winning {
	win_set_len = boards.len();
    }
    for draw in draws {
	match hashmap.get(&draw){
	    Some(vec) => {
		for (board_no, y, x) in vec {
		    boards[*board_no][*y][*x] = -1;
		    if check_win(boards[*board_no], (*board_no, *y, *x)) {
			winning_boards.insert(*board_no);
			sum = sum_board(boards[*board_no]);
			l_draw = draw;
			if first_winning {
			    break;
			} else {
			    if win_set_len == winning_boards.len() {
				break;
			    }
			}
		    }
		}
	    }
	    None => (),
	}
	if win_set_len == winning_boards.len() {
	    break;
	}
    }
    (sum, l_draw)
}

fn sum_board(board: [[i32; 5]; 5]) -> i32 {
    let mut sum = 0;
    for row in &board {
	for n in row {
	    if *n != -1 {
		sum += n;
	    }
	}
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
	include_str!("input_test.txt")
    }

    #[test]
    fn test_get_draws() {
	let (draws, _, _) = read_input(get_input());
	assert_eq!(vec!(7,4,9,5,11,17,23,2,0,14,21,
		       24,10,16,13,6,15,25,12,22,
		       18,20,8,19,3,26,1), draws);
    }

    #[test]
    fn test_hashmap() {
	let (_, _, hashmap) = read_input(get_input());
	assert_eq!(3, hashmap.get(&24).unwrap().len());
	assert!(hashmap.get(&24).unwrap().contains(&(0, 1, 4)));
	assert!(hashmap.get(&24).unwrap().contains(&(1, 3, 3)));
	assert!(hashmap.get(&24).unwrap().contains(&(2, 0, 3)));
	assert!(hashmap.get(&72).is_none());
	assert!(!hashmap.get(&24).unwrap().contains(&(2,2,2)));
	assert!(!hashmap.get(&24).unwrap().contains(&(2,2,8)));
	assert!(!hashmap.get(&24).unwrap().contains(&(5,2,1)));
    }

    #[test]
    fn test_first_winning_last_draw() {
	let (_, l_draw) = run(true, read_input(get_input()));
	assert_eq!(24, l_draw);
    }
    
    #[test]
    fn test_first_winning_sum() {
	let (sum, _) = run(true, read_input(get_input()));
	assert_eq!(188, sum);
    }

    #[test]
    fn test_last_winning_last_draw() {
	let (_, l_draw) = run(false, read_input(get_input()));
	assert_eq!(13, l_draw);
    }
    
    #[test]
    fn test_last_winning_sum() {
	let (sum, _) = run(false, read_input(get_input()));
	assert_eq!(148, sum);
    }
}

