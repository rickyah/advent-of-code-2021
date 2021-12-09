use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::HashMap;
use std::collections::HashSet;

// https://adventofcode.com/2021/day/8

pub struct Input{
	signals: Vec<Vec<String>>,
	digits:  Vec<Vec<String>>
}

#[aoc_generator(day8)]
pub fn parser(input: &str) -> Input{
	let mut signals : Vec<Vec<String>> = Vec::new();
	let mut digits : Vec<Vec<String>> = Vec::new();

	for line in input.lines() {
		let mut parts = line.split("|");

		let line_signals = parts.next().unwrap().split_whitespace().map(String::from).collect();
		let line_digits = parts.next().unwrap().split_whitespace().map(String::from).collect();

		signals.push(line_signals);
		digits.push(line_digits);
	}

	return Input {
		signals: signals,
		digits: digits
	}
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Input) -> u32 {
	let mut count = 0;

	for vec_4_digits in &input.digits {

		for digit in vec_4_digits {
			let char_count = digit.chars().count();
			match char_count {
				2 | 3 | 4 | 7 => count += 1,
				_ => ()
			}
		}
	}

	return count;
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Input) -> u32 {

	let mut digits_vector : Vec<u32> = Vec::new();

	for idx in 0..input.digits.len() {

		// in this hashmap we will map a digit to a signal set
		let mut digits : HashMap<u8, HashSet<char>> = HashMap::new();
		let mut signals_len_6 : Vec<HashSet<char>> = Vec::new();
		let mut signals_len_5 : Vec<HashSet<char>> = Vec::new();
		for signal_string in &input.signals[idx] {
			for signal in signal_string.split_whitespace() {
				let char_hashmap = HashSet::from_iter(signal.chars());
				let char_count = char_hashmap.len() as u8;

				// Digits 1,7,4,8 are inmmediate to locate
				if char_count == 2 { digits.entry(1).or_insert(char_hashmap); }
				else if char_count == 3 { digits.entry(7).or_insert(char_hashmap); }
				else if char_count == 4 { digits.entry(4).or_insert(char_hashmap); }
				else if char_count == 7 { digits.entry(8).or_insert(char_hashmap); }
				else if char_count == 6 { signals_len_6.push(char_hashmap)}
				else if char_count == 5 { signals_len_5.push(char_hashmap)}
				else {
					panic!("");
				}
			}
		}

		// I will use the difference operation between the signals to figure out
		// different digits

		// Digits formed with 5 signals
		let signals_4 = digits.get(&4).unwrap();
		// Digit 2 is the only for which
		// signals_digit_2 - signals_digit_4 = 3 signals
		let signals_2_idx = signals_len_5.iter()
			.position(|sig| sig.difference(&signals_4).count() == 3)
			.unwrap();
		let signals_2 = signals_len_5.swap_remove(signals_2_idx);

		// Digit 3 is the only for which
		// signals_digit_3 - signals_digit_2 = 1 signal
		let signals_3_idx = signals_len_5.iter()
			.position(|sig| sig.difference(&signals_2).count() == 1)
			.unwrap();
		let signals_3 = signals_len_5.swap_remove(signals_3_idx);

		// The last element in the vector are the signals for the digit 5
		let signals_5 = signals_len_5.swap_remove(0);

		// Digits formed with 6 signals
		let signals_1 = digits.get(&1).unwrap();
   		// Digit 6 is the only for which
		// signals_digit_6 - signals_digit_ 1 = 5 signals
		let signals_6_idx = signals_len_6.iter()
			.position( |sig| sig.difference(signals_1).count() == 5)
			.unwrap();
		let signals_6 = signals_len_6.swap_remove(signals_6_idx);

		// 0 - 5 should get us a hashset with 2 element
		// Digit 6 is the only for which
		// signals_digit_6 - signals_digit_ 4 = 2 signals
		let signals_9_idx = signals_len_6.iter()
			.position(|sig| sig.difference(&signals_4).count() == 2)
			.unwrap();
		let signals_9 = signals_len_6.swap_remove(signals_9_idx);
		// The last element in the vector are the signals for the digit 0
		let signals_0 = signals_len_6.swap_remove(0);

		digits.entry(2).or_insert(signals_2);
		digits.entry(3).or_insert(signals_3);
		digits.entry(5).or_insert(signals_5);
		digits.entry(6).or_insert(signals_6);
		digits.entry(9).or_insert(signals_9);
		digits.entry(0).or_insert(signals_0);

		let mut current_digit = 0_u32;
		let mut current_exponent = 4_u32;

		for digit_str in &input.digits[idx] {

			let digit_signals : HashSet<char> = HashSet::from_iter(digit_str.chars());

			let digit = digits.iter()
				.find_map(|(k, v)| if &digit_signals == v {Some(*k)} else { None })
				.unwrap() as u32;

			current_exponent -= 1_u32;
			let v =  digit * 10_u32.pow(current_exponent as u32);
			current_digit += v;
		}
		digits_vector.push(current_digit as u32);

	}
	return digits_vector.iter().sum();

}

mod tests {
	use super::*;

	const INPUT_LITERAL : &str =
"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |fgae cfgab fg bagce";

	#[test]
	fn test_day8_part1() {
		let input = parser(INPUT_LITERAL);
		let result = solve_part1(&input);
		assert_eq!(result, 26);
	}

	#[test]
	fn test_day8_part2() {
		let input = parser(INPUT_LITERAL);
		let result = solve_part2(&input);
		assert_eq!(result, 61229);
	}

}
