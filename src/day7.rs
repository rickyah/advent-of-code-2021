use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

// https://adventofcode.com/2021/day/7

pub struct Input {

}

#[aoc_generator(day7)]
pub fn parser(input: &str) -> Vec<i32>{
	return input.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Vec<i32>) -> u32 {
	fn distance_compute(distance:i32) -> i32 {
		return distance;
	}
	return solver(&input, distance_compute);
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Vec<i32>) -> u32 {
	fn distance_compute(distance:i32) -> i32 {
		return (distance * (distance + 1)) / 2;
	}
	return solver(&input, distance_compute);
}

pub fn solver(input: &Vec<i32>, distance_cost_fn: fn (i32) -> i32) -> u32 {
	let min = *input.iter().min().unwrap() as usize;
	let max = *input.iter().max().unwrap() as usize;
	
	let h_pos_lenght = max - min;
	let mut h_positions : Vec<i32> = vec![0; h_pos_lenght];

	for possible_pos in 0..h_pos_lenght {
		for sub_position in input {
			let distance = i32::abs(possible_pos as i32 - *sub_position as i32);

			// Divergent series 1+2+3+4...
			// n (n+1)
			// -------
			//    2
			//https://en.wikipedia.org/wiki/1_%2B_2_%2B_3_%2B_4_%2B_%E2%8B%AF
			h_positions[possible_pos as usize] += distance_cost_fn(distance);
		}
	}

	return *h_positions.iter().min().unwrap() as u32;
}

mod tests {
	use super::*;

	const INPUT_LITERAL : &str = "16,1,2,0,4,2,7,1,2,14";
 
	#[test]
	fn test_day7_part1() {
		let input = parser(INPUT_LITERAL);
		let result = solve_part1(&input);
		assert_eq!(result, 37);
	}

	#[test]
	fn test_day7_part2() {
		let input = parser(INPUT_LITERAL);
		let result = solve_part2(&input);
		assert_eq!(result, 168);
	}
	
}