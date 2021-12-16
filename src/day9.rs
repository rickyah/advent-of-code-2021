use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::VecDeque;
use std::collections::HashSet;

// https://adventofcode.com/2021/day/9

pub struct Input {
	matrix:Vec<Vec<u8>>
}

// (row, col)
type Coord = (u32,u32);

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

#[aoc(day9, part1)]
pub fn solve_part1(input: &Input) -> u32 {
	let coords = find_lowest_points_coords(&input.matrix);
	return coords.iter()
		.map(|c| input.matrix[c.0 as usize][c.1 as usize] as u32 + 1)
		.sum();
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Input) -> u32 {
	let coords = find_lowest_points_coords(&input.matrix);
	let mut basins = find_all_basins_sizes(&coords, &input.matrix);
	basins.sort();
	basins.reverse();
	let result = basins[0..3].iter().fold(1, |acc, b| acc * b);
	return result;
}

fn find_all_basins_sizes(lowest_points: &Vec<Coord>, matrix: &Vec<Vec<u8>>) -> Vec<u32> {
	return lowest_points.iter()
		.map(|p| first_bread_search(p, matrix).len() as u32)
		.collect();
}

fn first_bread_search(initial_coord: &Coord, matrix: &Vec<Vec<u8>>) -> Vec<Coord> {
	let height = matrix.len() as u32;
	let width = matrix[0].len() as u32;

	let mut frontier = VecDeque::<Coord>::new();
	let mut result = HashSet::<Coord>::new();
	result.insert(*initial_coord);
	frontier.push_back(*initial_coord);

	// I use a breadth-first search
	// https://www.redblobgames.com/pathfinding/a-star/introduction.html#breadth-first-search
	while !frontier.is_empty() {
		let current_coord = frontier.pop_front().unwrap();

		let neighbourhoods = get_neighbourhoods(&current_coord , width, height);
		for n in neighbourhoods{
			let n_val = matrix[n.0 as usize][n.1 as usize];

			if n_val != 9 {
				if !result.contains(&n) {
					result.insert(n);
					frontier.push_back(n)
				};
			}
		}
	}

	return result.into_iter().collect();
}

fn find_lowest_points_coords(matrix: &Vec<Vec<u8>>) -> Vec<Coord> {
	let height = matrix.len() as u32;
	let width = matrix[0].len() as u32;
	let mut result : Vec<Coord> = Vec::new();

	for (row, rowline) in matrix.iter().enumerate() {

		for (col, _) in rowline.iter().enumerate() {
			let current_coord = (row as u32, col as u32);
			let coords = get_neighbourhoods(&current_coord, width, height);
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

fn get_neighbourhoods(initial: &Coord, width:u32, height:u32) -> Vec<Coord> {
	let row = initial.0 as i32;
	let col = initial.1 as i32;
	let width = width as i32;
	let height = height as i32;

	let neighbourhoods = [
		(row -1, col),
		(row,col -1),
		(row,col +1),
		(row +1, col),
	];

    let result = neighbourhoods.into_iter()
	 .filter(|coord| coord.0 >= 0 && coord.0 < height && coord.1 >= 0 && coord.1 < width)
	 .map(|c| (c.0 as u32, c.1 as u32 ))
	 .collect();

	return result;
}

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

	#[test]
	fn test_day9_part2() {
		let input = parser(INPUT_LITERAL);
		let result = solve_part2(&input);
		assert_eq!(result, 1134);
	}

}