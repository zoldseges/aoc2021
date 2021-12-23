use std::sync::mpsc;
use std::thread;
use aoc2021::init as init;

fn main() {
    let vec = init();
    let (tx, rx) = mpsc::channel();
    //TODO implement spawning 3-4 threads instead
    for mut puzzle in vec {
	let thread_tx = tx.clone();
	thread::spawn(move || {
	    puzzle.solve();
	    thread_tx.send(puzzle).unwrap();
	});
    }
    drop(tx);
    let mut vec = Vec::new();
    for received in rx {
	vec.push(received);
    }

    for puzzle in vec {
	if let (None, None) = (puzzle.get_solution_part1(), puzzle.get_solution_part2()) {
	    continue;
	}
	let mut name = "Untitled";
	if let Some(n) = puzzle.get_name(){
	    name = n;
	}
	println!("\nDay {:2} - {}", puzzle.get_day(), name);
	match puzzle.get_solution_part1() {
	    Some(s) => println!("{:9}{} {}", ' ', "Part 1:", s),
	    None => println!("{:9}{} {}", ' ', "Part 1:", "Unsolved"),
	}
	match puzzle.get_solution_part2() {
	    Some(s) => println!("{:9}{} {}", ' ', "Part 2:", s),
	    None => println!("{:9}{} {}", ' ', "Part 2:", "Unsolved"),
	}
    }
}
