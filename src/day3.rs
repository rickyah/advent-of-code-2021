use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

//  https://adventofcode.com/2021/day/3


pub struct ParsedInput {
	// Assumes that all numbres in the input data are represented
	// by strings with the same number of binary digits
	num_bits : u32,

	// List of numbers once the input string is parsed
	numbers : Vec<u32>,
}

pub struct PowerConsumptionRates {
	gamma : u32,
	epsilon : u32
}

pub struct LifeSupportRates {
	oxygen_generator : u32,
	co2_scrubble : u32
}


#[aoc_generator(day3)]
pub fn parse(input: &str) -> ParsedInput {
	let first_line_chars_num = input.lines().next().unwrap().len();
	let numbers = input.lines().map( |line| u32::from_str_radix(&line, 2).unwrap()).collect();

	return ParsedInput {
		num_bits: first_line_chars_num as u32,
		numbers: numbers
	} 
}

#[aoc(day3, part1)]
pub fn compute_day3_part1(input : &ParsedInput) -> u32 {
	let set_bits_list = count_set_bits_by_position(&input);

	let rates = compute_power_consumption_rates(set_bits_list, input.numbers.len());

	return rates.gamma * rates.epsilon;
}

#[aoc(day3, part2)]
pub fn compute_day3_part2(input : &ParsedInput) -> u32 {
	let mut oxygen_working_set = input.numbers.clone();
	let mut co2_working_set = input.numbers.clone();
	let num_bits = input.num_bits as usize;

	while oxygen_working_set.len() > 1 {
		for idx in (0..num_bits).rev() {
			// Keep separate list for elements with the current bit position set or unset 
			let mut elements_with_bit_unset : Vec<u32> = Vec::with_capacity(oxygen_working_set.len());
			let mut elements_with_bit_set : Vec<u32> = Vec::with_capacity(oxygen_working_set.len());

			for number in oxygen_working_set {
				if number & (1 << idx) != 0 {
					elements_with_bit_set.push(number);
				} else {
					elements_with_bit_unset.push(number);
				}
			}

			if elements_with_bit_set.len() > elements_with_bit_unset.len() {
				oxygen_working_set = elements_with_bit_set.clone();
			} else if elements_with_bit_unset.len() > elements_with_bit_set.len() {
				oxygen_working_set = elements_with_bit_unset.clone();
			} else {
				oxygen_working_set = elements_with_bit_set.clone();
			}

			if oxygen_working_set.len() == 1 { break; }
		}
	}
	while co2_working_set.len() > 1 {
		for idx in (0..num_bits).rev() {
			let mut elements_with_bit_unset : Vec<u32> = Vec::with_capacity(co2_working_set.len());
			let mut elements_with_bit_set : Vec<u32> = Vec::with_capacity(co2_working_set.len());

			for number in co2_working_set {
				if number & (1 << idx) != 0 {
					elements_with_bit_set.push(number);
				} else {
					elements_with_bit_unset.push(number);
				}
			}

			if elements_with_bit_set.len() < elements_with_bit_unset.len() {
				co2_working_set = elements_with_bit_set.clone();
			} else if elements_with_bit_unset.len() < elements_with_bit_set.len() {
				co2_working_set = elements_with_bit_unset.clone();
			} else {
				co2_working_set = elements_with_bit_unset.clone();
			}

			if co2_working_set.len() == 1 { break; }
		}
	}

	return co2_working_set[0] * oxygen_working_set[0];
}

fn compute_power_consumption_rates(set_bits: Vec<u32>, total_numbers: usize) -> PowerConsumptionRates {

	let half_numbers = (total_numbers / 2) as u32;
	let mut gamma = 0;
	let mut epsilon = 0;

	for (idx, bit_count) in set_bits.iter().enumerate() {
		let idx = set_bits.len() - idx -1;
		let bit_position = 1 << idx;
		if *bit_count > half_numbers {
			gamma = gamma | bit_position ;
		} else {
			epsilon = epsilon | bit_position ;
		}
	}

	return PowerConsumptionRates {gamma: gamma, epsilon: epsilon}; 
}

pub fn count_set_bits_by_position(input : &ParsedInput) -> Vec<u32> {
	let num_bits = input.num_bits as usize;
	let mut bit_set_positions = vec! [0; num_bits];

	for number in &input.numbers {
		for idx in 0..num_bits {
			if number & (1 << idx) != 0 {
				let bit_pos = num_bits - idx - 1;
				bit_set_positions[bit_pos] = bit_set_positions[bit_pos] + 1;
			}
		}
	}

	return bit_set_positions;
}

#[cfg(test)]
mod tests {
	use super::*;
	
	const INPUT_LITERAL : &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

	const INPUT_NUMBERS : [u32;12] = [
		0b00100,
		0b11110,
		0b10110,
		0b10111,
		0b10101,
		0b01111,
		0b00111,
		0b11100,
		0b10000,
		0b11001,
		0b00010,
		0b01010,
	];

	#[test]
	fn test_count_bits_numbers() {
		let input = ParsedInput {
			num_bits: 5,
			numbers:  INPUT_NUMBERS.iter().map(|n| *n).collect(),
		};

		let result = count_set_bits_by_position(&input);

		assert_eq!(result[result.len()-5..], vec![7,5,8,7,5]);
	}

	#[test]
	fn test_12_bits_str_to_int32()
	{
		const num_5 : i32 = 0b0000_0000_0101;
		const str_num_5 : &str = "000000000101";

		assert_eq!(num_5, 5 as i32);
		let parsed = i32::from_str_radix(str_num_5, 2).unwrap();
		assert_eq!(parsed, num_5);
	}
	#[test]
	fn test_day3_parser() {
		let parsed_data = parse(INPUT_LITERAL);
		
		assert_eq!(&parsed_data.numbers[..], &INPUT_NUMBERS[..]);
	} 


	#[test]
	fn test_day2_part1() {
		let input = ParsedInput {
			num_bits: 5,
			numbers:  INPUT_NUMBERS.iter().map(|n| *n).collect(),
		};

		assert_eq!(compute_day3_part1(&input), 198);
	}

	#[test]
	fn test_day2_part2() {
		let input = ParsedInput {
			num_bits: 5,
			numbers:  INPUT_NUMBERS.iter().map(|n| *n).collect(),
		};

		assert_eq!(compute_day3_part2(&input), 230);
	}
}