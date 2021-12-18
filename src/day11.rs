use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::HashSet;

// https://adventofcode.com/2021/day/11

pub struct Input {
	matrix:Vec<Vec<u8>>
}

// (row, col)
type Coord = (u32,u32);

#[aoc_generator(day11)]
pub fn parser(input: &str) -> Input{
	let mut matrix:Vec<Vec<u8>> = Vec::new();
	for line in input.lines() {
		matrix.push(line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect());
	}

	return Input {
		matrix
	};
}


#[aoc(day11, part1)]
pub fn solve_part1(input: &Input) -> u32 {
	let mut matrix = input.matrix.to_owned();

	return simulate_steps(&mut matrix, 100)
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Input) -> u32 {
	let mut matrix = input.matrix.to_owned();

	return find_first_sync(&mut matrix)
}

fn find_first_sync(matrix: &mut Vec<Vec<u8>>) -> u32 {

	let mut step = 0;

	let width = matrix[0].len() as u32;
	let height = matrix.len() as u32;
	let matrix_size = width * height;

	// Loop hoping condition will be met because problem says so
	loop {
		step += 1;
		let flashes_count = increment_energy_levels(matrix);
		if flashes_count == matrix_size {return step;}
	}
}

fn simulate_steps(matrix: &mut Vec<Vec<u8>>, step:i8) -> u32 {
	// Termination Condition
	if step <= 0 {return 0;}

	// 1. increment all by one
	// 2. find elements with value 10
	// 3. modify those elements to be 0, get its neighborhoods, increase counter
	// 4. increment neighborhoods by 1, goto 2.


	let flashes_count = increment_energy_levels(matrix);

	return flashes_count + simulate_steps(matrix, step -1);
}

fn increment_energy_levels(matrix: &mut Vec<Vec<u8>>) -> u32 {
	let mut to_flash = Vec::new();

	let mut row_idx = 0_u32;

	let mut flashed = HashSet::new();
	for row in matrix.iter_mut() {
		let mut col_idx = 0_u32;
		for element in row.iter_mut() {
			*element += 1;
			if *element > 9_u8 {
				flashed.insert( (row_idx, col_idx) );
				to_flash.push( (row_idx, col_idx) );
				*element = 0;
			}
			 col_idx += 1;
		}
		row_idx += 1;
	}

	process_flashed(matrix, &to_flash, &mut flashed);

	return flashed.len() as u32;
}

fn process_flashed(
	matrix: &mut Vec<Vec<u8>>,
	to_flash: &Vec<Coord>,
	flashed: &mut HashSet<Coord> ) {

	if to_flash.is_empty() {return}
	let width = matrix[0].len() as u32;
	let height = matrix.len() as u32;

	let mut new_to_flash = Vec::<Coord>::new();

	for coord in to_flash.iter() {
		// octopush that flash give energy to neighbourhoods
		let neighbourhoods = get_neighbourhoods(coord, width, height);
		for neighbourhood in neighbourhoods {
			if flashed.contains(&neighbourhood) { continue; }

			let element = &mut matrix[neighbourhood.0 as usize][neighbourhood.1 as usize];

			*element += 1;

			// nighbourhoods could flash too
			if *element > 9_u8  {
				new_to_flash.push(neighbourhood);
				flashed.insert(neighbourhood);
				*element = 0;
			}
		}

	}

	process_flashed(matrix, &new_to_flash, flashed);
}

fn get_neighbourhoods(initial: &Coord, width:u32, height:u32) -> Vec<Coord> {
	let row = initial.0 as i32;
	let col = initial.1 as i32;
	let width = width as i32;
	let height = height as i32;

	let neighbourhoods = [
		(row -1, col -1), (row -1, col), (row -1, col +1),
		(row +0, col -1),				 (row +0, col +1),
		(row +1, col -1), (row +1, col), (row +1, col +1)
	];

    let result = neighbourhoods.into_iter()
	 .filter(|coord| coord.0 >= 0 && coord.0 < height && coord.1 >= 0 && coord.1 < width)
	 .map(|c| (c.0 as u32, c.1 as u32 ))
	 .collect();

	return result;
}

fn print_matrix(matrix: & Vec<Vec<u8>>) {

	println!("");

	for row in matrix.iter() {
		for element in row.iter() {
			print!(" {:?} ", element);
		}
		println!("");
	}

	println!("");
}

mod tests {
	use super::*;

	const INPUT_LITERAL : &str =
"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";


	#[test]
	fn test_day11_part1() {
		let input = parser(INPUT_LITERAL);
		let result = solve_part1(&input);
		assert_eq!(result, 1656);
	}

	#[test]
	fn test_day11_part2() {
		let input = parser(INPUT_LITERAL);
		let result = solve_part2(&input);
		assert_eq!(result, 195);
	}

}