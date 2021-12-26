use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::HashMap;

// https://adventofcode.com/2021/day/14

pub struct Input {
   polymer: String,
   insertions: HashMap<String, String>
}

#[aoc_generator(day14)]
pub fn parser(input: &str) -> Input{
	let mut lines = input.lines();
	let polymer = String::from(lines.next().unwrap());
	lines.next();
	let insertions : HashMap<_,_> = lines.map(|line| {
		let mut split = line.split(" -> ");
		return
			(
				String::from(split.next().unwrap()),
				String::from(split.next().unwrap())
			);

	}).into_iter().collect();

	return Input {
		polymer,
		insertions,
	};
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &Input) -> u32 {

	for _ in 0..10 {

		for c in

	}
	return 0;
}

// #[aoc(day14, part2)]
// pub fn solve_part2(input: &Input) -> u32 {
// 	return 0;
// }

mod tests {
	use super::*;

	const INPUT_LITERAL : &str =
"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

	#[test]
	fn test_day14_part1() {
		let input = parser(INPUT_LITERAL);
		let result = solve_part1(&input);
		assert_eq!(result, 1588);
	}

//	#[test]
//	fn test_day14_part2() {
//		let input = parser(INPUT_LITERAL);
//		let result = solve_part2(&input);
//		assert_eq!(result, []);
//	}

}