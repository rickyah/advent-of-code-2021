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
type Coord = (u32,u32);


// Notes:
// 0 and 9 are lowest / highes points automatically
// 0 does not add to the solution, 9 does not need to be
// compared to the adjacent, so both can be skipped
#[aoc(day9, part1)]
pub fn solve_part1(input: &Input) -> u32 {
	let coords = find_lowest_points_coords(&input.matrix);
	return coords.iter()
		.map(|c| input.matrix[c.0 as usize][c.1 as usize] as u32)
		.sum();
}

fn find_lowest_points_coords(matrix: &Vec<Vec<u8>>) -> Vec<Coord> {
	let height = matrix.len() as u32;
	let width = matrix[0].len() as u32;
	let mut result : Vec<Coord> = Vec::new();

	for (row, rowline) in matrix.iter().enumerate() {

		for (col, _) in rowline.iter().enumerate() {
			let current_coord = (row as u32, col as u32);
			let coords = get_coordinates(current_coord, width, height);
			let value_to_check = matrix[row][col];

			let min_surrounding_value = coords.iter()
				.map(|c| matrix[c.0 as usize][c.1 as usize])
				.fold(std::u8::MAX, |acc, v| if v < acc { return v } else { return acc } );

			if value_to_check < min_surrounding_value {
				result.push(current_coord);
			}
		}
	}

	return result;
}

fn get_coordinates(initial: Coord, width:u32, height:u32) -> Vec<Coord> {
	let row = initial.0 as i32;
	let col = initial.1 as i32;
	let width = width as i32;
	let height = height as i32;

	let v = [
		(row -1, col),
		(row,col -1),
		(row,col +1),
		(row +1, col),
	];

    let result = v.into_iter()
	 .filter(|coord| coord.0 >= 0 && coord.0 < height && coord.1 >= 0 && coord.1 < width)
	 .map(|c| (c.0 as u32, c.1 as u32 ))
	 .collect();

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