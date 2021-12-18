use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::HashMap;

// https://adventofcode.com/2021/day/12

pub struct Input {
	nodes: HashMap<String, Vec<String>>
}

#[aoc_generator(day12)]
pub fn parser(input: &str) -> Input{
	let mut nodes : HashMap<String, Vec<String>> = HashMap::new();
	for line in input.lines() {

		let mut it = line.split('-');
		let parts = (it.next().unwrap(), it.next().unwrap());
		let parts = (String::from(parts.0), String::from(parts.1));

		let v = nodes.entry(parts.0.clone()).or_insert(Vec::new());
		v.push(parts.1.clone());

		let v = nodes.entry(parts.1.clone()).or_insert(Vec::new());
		v.push(parts.0.clone())
	}

	// println!("Nodes:");
	// println!("{:?}", nodes);
	// println!("");
	return Input {nodes};
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &Input) -> u32 {
	let paths = compute_all_paths(&input.nodes);
	// println!("paths:");
	// println!("{:?}", paths);
	return paths.len() as u32;
}

// #[aoc(day12, part2)]
// pub fn solve_part2(input: &Input) -> u32 {
// 	return 0;
// }

fn compute_all_paths(nodes: &HashMap<String, Vec<String>>,) -> Vec<Vec<String>> {
	let mut path = Vec::new();
	return compute_subpath_for_node(&"start".to_string(), &nodes, &mut path);
}


fn compute_subpath_for_node(
	node: &String,
	nodes: &HashMap<String, Vec<String>>,
	path: &mut Vec<String>)
	-> Vec<Vec<String>> {
	// If node is lowercase we need to ensure that this node is not
	// already in the path
	// input: a node

	let mut paths : Vec<Vec<String>> = Vec::new();
	// if node is 'end' add it to the path and finish
	if node == "end" {
		path.push(node.clone());
		paths.push(path.to_vec());
		return paths;
	}
	// if node is lowercase and contained in the path, finish
	if node.find(char::is_uppercase) == None {
		if path.contains(&node) {
			return paths;
		}
	}

	path.push(node.clone());
	let childs = nodes.get(node).unwrap();
	for child in childs {
		let mut sub_path = path.clone();
		paths.append(&mut compute_subpath_for_node(&child, nodes, &mut sub_path));
	}

	return paths;
}

mod tests {
	use super::*;

	const INPUT_LITERAL1 : &str =
"start-A
start-b
A-c
A-b
b-d
A-end
b-end";

const INPUT_LITERAL2 : &str =
"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
	#[test]
	fn test_day12_part1_1() {
		let input = parser(INPUT_LITERAL1);
		let result = solve_part1(&input);
		assert_eq!(result, 10);
	}

	#[test]
	fn test_day12_part1_2() {
		let input = parser(INPUT_LITERAL2);
		let result = solve_part1(&input);
		assert_eq!(result, 19);
	}

//	#[test]
//	fn test_day12_part2() {
//		let input = parser(INPUT_LITERAL);
//		let result = solve_part2(&input);
//		assert_eq!(result, []);
//	}

}