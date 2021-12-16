use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

// https://adventofcode.com/2021/day/9

pub struct Input {
	matrix:Vec<Vec<u8>>
}

#[aoc_generator(day9)]
pub fn parser(input: &str) -> Input{
	let mut matrix:Vec<Vec<u8>> = Vec::new();
	for line in input.lines() {
		matrix.push(line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect());
	}
	return Input {
		matrix
	};
}


// (row, col)
type Coord = (i32,i32);


// Notes:
// 0 and 9 are lowest / highes points automatically
// 0 does not add to the solution, 9 does not need to be
// compared to the adjacent, so both can be skipped
#[aoc(day9, part1)]
pub fn solve_part1(input: &Input) -> u32 {

	let mut sum = 0_u32;
	let height = input.matrix.len() as u32;
	let width = input.matrix[0].len() as u32;

	println!("({},{})", width, height);
	for (row, rowline) in input.matrix.iter().enumerate() {

		for (col, _) in rowline.iter().enumerate() {
			let coords = get_coordinates( (row as i32, col as i32), width, height );
			let value_to_check = input.matrix[row][col];

			let min_surrounding_value = coords.iter()
				.map(|c| input.matrix[c.0 as usize][c.1 as usize])
				.fold(std::u8::MAX, |acc, v| if v < acc { return v } else { return acc } );

			if value_to_check < min_surrounding_value {
				print!("{}", value_to_check);
				sum += 1 + value_to_check as u32;
			} else {
				print!("*");
			}
		}
		println!(" ");
	}
	return sum;
}
fn get_coordinates(initial: Coord, width:u32, height:u32) -> Vec<Coord> {
	let v  = [
		(initial.0 -1, initial.1),
		(initial.0, initial.1 -1),
		(initial.0, initial.1 +1),
		(initial.0 +1, initial.1),
	];



	let width = width as i32;
	let height = height as i32;
    let result = v.into_iter()
	 .filter(|coord| coord.0 >= 0 && coord.0 < height && coord.1 >= 0 && coord.1 < width)
	 .collect();

	// println!("i:{:?}", initial);
	// println!("v:{:?}", v);
	// println!("r:{:?}", result);
	return result;
}
// #[aoc(day9, part2)]
// pub fn solve_part2(input: &Input) -> u32 {
// 	return 0;
// }

mod tests {
	use super::*;

	const INPUT_LITERAL : &str =
"2199943210
3987894921
9856789892
8767896789
9899965678";

	#[test]
	fn test_day9_part1() {
		let input = parser(INPUT_LITERAL);
		let result = solve_part1(&input);
		assert_eq!(result, 15);
	}

//	#[test]
//	fn test_day9_part2() {
//		let input = parser(INPUT_LITERAL);
//		let result = solve_part2(&input);
//		assert_eq!(result, []);
//	}

}