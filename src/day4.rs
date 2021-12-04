use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

//  https://adventofcode.com/2021/day/4

pub struct Input {
}

#[aoc_generator(day4)]
pub fn parser(input: &str) -> Input{
	return Input{};
}

// How many measurements are larger than the previous measurement?
#[aoc(day4, part1)]
pub fn solve_part1(input:&Input) -> u32 {
	return 0 
}

// Consider sums of a three-measurement sliding window. 
// How many sums are larger than the previous sum?
#[aoc(day4, part2)]
pub fn solve_part2(input:&Input) -> u32 {
	return 0;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_day4_part1() {
		assert!(false);
	}

	#[test]
	fn test_day4_part2() {
		assert!(false);
	}
	
}