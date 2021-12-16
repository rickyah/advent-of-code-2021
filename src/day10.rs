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


#[aoc(day10, part2)]
pub fn solve_part2(input: &Input) -> u32 {
	return 0;
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

}