use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::HashMap;

//  https://adventofcode.com/2021/day/5


#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Point {
	x: u32,
	y: u32
}

#[derive(PartialEq, Eq, Debug)]
pub struct Line {
	p1 : Point,
	p2 : Point,
}

pub struct Input {
	lines : Vec<Line>
}

#[aoc_generator(day5)]
pub fn parser(input: &str) -> Input{
	let mut lines : Vec<Line> = Vec::new();
	for line in input.lines() {
		let coords : Vec<u32> = 
			line.split(" -> ")
			.map(|point| point.split(","))
			.flatten()
			.map(|coord| u32::from_str_radix(coord, 10).unwrap())
			.collect();

		lines.push(Line { 
			p1: Point { x: coords[0], y: coords[1] },
			p2: Point { x: coords[2], y: coords[3] },
		})
	}
	return Input { lines: lines };
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Input) -> u32 {
	let lines : Vec<&Line> = input.lines.iter()
		.filter(|line| line.p1.x == line.p2.x || line.p1.y == line.p2.y)
		.collect();
	
	let mut points_freq : HashMap<Point, u32> = HashMap::new();
	for line in lines {
		let p = Point {x: line.p1.x, y: line.p1.y};
		points_freq.entry(p).and_modify(|f| *f +=1).or_insert(1);
		
		
		let x_steps = line.p2.x as i32 - line.p1.x as i32;
		let x_inc:i32 = if x_steps < 0 {-1} else {1};
		let x_steps = i32::abs(x_steps);

		for step in 1..=x_steps {
			let x : i32 = line.p1.x as i32 + (x_inc * step);
			let p = Point {x: x as u32, y: line.p1.y};
			points_freq.entry(p).and_modify(|f| *f +=1).or_insert(1);
		}

		let y_steps : i32 = line.p2.y as i32 - line.p1.y as i32;
		let y_inc:i32 = if y_steps < 0 {-1} else {1};
		let y_steps = i32::abs(y_steps);

		for step in 1..=y_steps {
			let y : i32 = line.p1.y as i32 + (y_inc * step);
			let p = Point {x: line.p1.x, y: y as u32};
			points_freq.entry(p).and_modify(|f| *f +=1).or_insert(1);
		}
	}
	

	let v : Vec<&Point> = points_freq.iter()
		.filter_map(|(key, &val)| if val >= 2 { Some(key) } else { None })	
		.collect();
	let result = v.len() as u32;
	return result; 
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Input) -> u32 {
	return 0;
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT_LITERAL : &str = 
"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

	#[test]
	fn test_day5_parser() {
		let input = parser(INPUT_LITERAL);
		assert_eq!(input.lines.len(), 10);
		assert_eq!(input.lines[0], Line { p1: Point{ x: 0, y:9 }, p2: Point { x: 5, y: 9} });
		assert_eq!(input.lines[9], Line { p1: Point{ x: 5, y:5 }, p2: Point { x: 8, y: 2} });
	}

	#[test]
	fn test_day5_part1() {
		let input = parser(INPUT_LITERAL);
		let result = solve_part1(&input);

		assert_eq!(result, 5);
	}
	
	#[test]
	fn test_day5_part2() {
		assert!(false);
	}

}