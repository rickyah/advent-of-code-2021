use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

// https://adventofcode.com/2021/day/10

pub struct Input {
	lines: Vec<String>
}

#[aoc_generator(day10)]
pub fn parser(input: &str) -> Input{
	let r = input.lines().map(|s| s.to_string()).collect();
	return Input {lines: r }
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Input) -> u32 {

	let mut count_invalid_parenthesis = 0;
	let mut count_invalid_squares = 0;
	let mut count_invalid_brackets = 0;
	let mut count_invalid_angles = 0;

	for line in &input.lines {
		match check_line_invalid(&line){
			Some(c) => {
				match c {
					')' => count_invalid_parenthesis += 1,
					']' => count_invalid_squares += 1,
					'}' => count_invalid_brackets += 1,
					'>' => count_invalid_angles += 1,
					_ => (),
				}
			}
			None => ()
		}
	}

	let result = 0
		+ count_invalid_parenthesis * 3
		+ count_invalid_squares * 57
		+ count_invalid_brackets * 1197
		+ count_invalid_angles * 25137;

	return result;
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Input) -> u64 {
	let filtered : Vec<&String> = input.lines.iter()
		.filter( |l| check_line_invalid(&l) == None)
		.collect();

	let completion_sequences : Vec<Vec<char>> = filtered.iter()
		.map(|line| get_completion_sequence(&line) )
		.collect();

	fn get_points(c :&char) -> u64 {
		return match c {
			')' => 1,
			']' => 2,
			'}' => 3,
			'>' => 4,
			_ => 0
		}
	}

	let mut points : Vec<u64> = completion_sequences.iter().map(|sequence| {
		sequence.iter().fold(0_u64, |acc, c| (acc * 5) + get_points(c))
	})
	.collect();

	points.sort();

	let middle = points.len() / 2;
	let result = points[middle];

	return result;
}

fn check_line_invalid(line: &String) -> Option<char> {
	let mut stack : Vec<char> = Vec::new();
	for c in line.chars() {

		match c {
			'{' | '[' | '(' | '<' => stack.push(c),
			'}' | ']' | ')' | '>' => {
				let last_element = stack.pop().unwrap();
				match (last_element, c) {
					('(', ')') => continue,
					('[', ']') => continue,
					('{', '}') => continue,
					('<', '>') => continue,
					_ => return Some(c),
				}
			},
			_ => ()
		}
	}
	None
}

fn get_completion_sequence(line: &String) -> Vec<char> {
	let mut stack : Vec<char> = Vec::new();
	for c in line.chars() {

		match c {
			'{' | '[' | '(' | '<' => stack.push(c),
			'}' | ']' | ')' | '>' => { stack.pop(); () },
			_ => ()
		}
	}

	fn get_replacement(c: &char) -> Option<char> {
		return match c {
			'{' => Some('}'),
			'[' => Some(']'),
			'(' => Some(')'),
			'<' => Some('>'),
			_ => None
		}
	}

	stack.reverse();
	for c in stack.iter_mut() {
		*c = get_replacement(&*c).unwrap();
	}

	return stack;
}

mod tests {
	use super::*;

	const INPUT_LITERAL : &str =
"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

	#[test]
	fn test_day10_part1() {
		let input = parser(INPUT_LITERAL);
		let result = solve_part1(&input);
		assert_eq!(result, 26397);
	}

	#[test]
	fn test_day10_part2() {
		let input = parser(INPUT_LITERAL);
		let result = solve_part2(&input);
		assert_eq!(result, 288957);
	}

	#[test]
	fn tt() {
		let v = [']', ')', ')', ')', '}', '}', '}', '}', ']', ')', '>', '>', '}', ')'];
fn get_points(c :&char) -> u32 {
		return match c {
			')' => 1,
			']' => 2,
			'}' => 3,
			'>' => 4,
			_ => 0
		}
	}
		let mut result = 0;
		for e in v {
			result *= 5;
			result += get_points(&e);
		}
		println!("{}", result);
	}
}