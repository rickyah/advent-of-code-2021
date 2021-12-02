use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Coord {
	x: i32,
	depth: i32
}

#[aoc_generator(day2)]
pub fn parser(input: &str) -> Vec<Coord> {
	
	let lines = input.lines();

	let mut result : Vec<Coord> = Vec::new();
	// parse each line as an int
	for line in lines {
		let line_elements : Vec<&str> = line.split_whitespace().take(2).collect();
		let command = line_elements[0];
		let value = line_elements[1].parse::<i32>().unwrap();

		match command {
			"forward" => result.push(Coord { x: value, depth: 0 }),
			"down" => result.push(Coord {x: 0, depth: value} ),
			"up" => result.push(Coord {x: 0, depth: -value} ),
			_ => panic!("Unknown command {}", command),
		}
	}
	return result;
}

#[aoc(day2, part1)]
pub fn compute_day2_part1(coords : &Vec<Coord>) -> i32 {
	return 0;
}

#[aoc(day2, part2)]
pub fn compute_day2_part2(coords : &Vec<Coord>) -> i32 {
	return 0;
}

#[cfg(test)]
mod tests {
	use super::*;
	
	const input_literal : &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

	const input_coords : [Coord;6] = [
		Coord{ x:5 , depth:0 },
		Coord{ x:0 , depth:5 },
		Coord{ x:8 , depth:0 },
		Coord{ x:0 , depth:-3 },
		Coord{ x:0 , depth:8 },
		Coord{ x:2 , depth:0 }
	];
	#[test]
	fn test_day2_parser() {
		let coords = parser(input_literal);

		assert_eq!(&coords[..], &input_coords[..]);
	}

	#[test]
	fn test_day2_part1() {
	}

	#[test]
	fn test_day2_part2() {
	}
}