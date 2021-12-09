use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

// https://adventofcode.com/2021/day/8

pub struct Input{
	signals: Vec<String>,
	digits: Vec<String>
}

#[aoc_generator(day8)]
pub fn parser(input: &str) -> Input{
	let mut d : Vec<String> = Vec::new();
	let mut s : Vec<String> = Vec::new();
	
	for line in input.lines() {
		let mut parts = line.split("|");
		
		s.push(String::from(parts.next().unwrap()));
		d.push(String::from(parts.next().unwrap()));
	}

	return Input { 
		signals: s,
		digits: d
	}
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Input) -> u32 {
	let mut count = 0;
	for digit_4 in &input.digits {

		for digit_1 in digit_4.split_whitespace() {
			let char_count = digit_1.chars().count();
			match char_count {
				2 | 3 | 4 | 7 => count += 1,
				_ => ()
			}
		}
	}

	return count;
}

// #[aoc(day8, part2)]
// pub fn solve_part2(input: &Input) -> u32 {
// 	return 0;
// }

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

//	#[test]
//	fn test_day8_part2() {
//		let input = parser(INPUT_LITERAL);
//		let result = solve_part2(&input);
//		assert_eq!(result, []);
//	}
	
}
