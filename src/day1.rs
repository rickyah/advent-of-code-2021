use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day1)]
pub fn parser(input: &str) -> Vec<i32> {
	
	// parse each line as an int
	return input.lines()
		.map( |l| l.parse::<i32>().unwrap() )
		.collect();
}

// How many measurements are larger than the previous measurement?
#[aoc(day1, part1)]
pub fn compute_part1(measurement_list : &[i32]) -> i32 {
	let mut count = 0;

	let mut idx = 1;
	while idx < measurement_list.len() {
		if measurement_list[idx] > measurement_list[idx-1] {
			count = count + 1
		}
		idx = idx + 1
	}

	return count 
}

// Consider sums of a three-measurement sliding window. 
// How many sums are larger than the previous sum?
#[aoc(day1, part2)]
pub fn compute_part2(measurement_list : &[i32]) -> i32 {
	let grouped_list_length = measurement_list.len() - 2;
	let mut grouped_list : Vec<i32> = Vec::with_capacity(grouped_list_length);

	let mut idx = 2;
	while idx < measurement_list.len() {
		let value =  measurement_list[idx-2] + measurement_list[idx-1] + measurement_list[idx];

		grouped_list.push(value);
		
		idx = idx + 1
	}

	return compute_part1(&grouped_list);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_day1_part1_even_size() {
		let input: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
		assert_eq!(compute_part1(&input), 7);
	}

	#[test]
	fn test_day1_part1_odd_size() {
		let input: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260];
		assert_eq!(compute_part1(&input), 6);
	}
	
	#[test]
	fn test_day1_part2_even_size() {
		let input: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
		// grouped by 3: [607,618,618,617,647,716,769]>
		assert_eq!(compute_part2(&input), 5);
	}

	#[test]
	fn test_day1_part2_odd_size() {
		let input: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260];
		// grouped by 3: [607,618,618,617,647,716,769]>
		assert_eq!(compute_part2(&input), 4);
	}	
}