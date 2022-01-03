use std::collections::HashMap;

pub fn name() -> Option<String> {
    Some("Extended Polymerization".to_string())
}

pub fn solve_p1() -> Option<String> {
    None
}

pub fn solve_p2() -> Option<String> {
    None
}

struct Input {
    base: String,
    rules: HashMap<String, String>,
}

impl Input {
    fn new(input: &str) -> Input {
        let mut base: String = String::new();
        let mut rules: HashMap<String, String> = HashMap::new();
        let mut lines = input.lines();
        base = lines.next().unwrap().to_string();
        while let Some(t) = lines.next() {
            let mut splitline = t.split("->");
            let mut from = String::new();
            match splitline.next() {
                None => continue,
                Some(t) if t.len() < 2 => {
		    continue;
                }
		Some(t) => {
                    from = t.to_string();
		}
            }
            let to = splitline.next().unwrap().to_string();
            rules.insert(from, to);
        }
        Input { base, rules }
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
        let input = Input::new(get_input());
        panic!();
    }
}
