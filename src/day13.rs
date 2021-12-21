use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::fmt;

// https://adventofcode.com/2021/day/13

type Coord = (u16, u16);

#[derive(Debug)]
enum FoldAxis {
	X(u16),
	Y(u16)
}

pub struct Paper {
	grid: Vec<Vec<bool>>,
}

impl Paper {

	fn default() -> Self {
		let mut grid = Vec::new(); let mut t = Vec::new();
		t.resize(1, false);
		grid.resize(1, t);

		Self { grid }
	}

	fn init_with_points(points: &Vec<Coord> ) -> Self{
		// Size of the grid
		let mut height = 0;
		let mut width = 0;

		for point in points {
			width = std::cmp::max(width, point.0 as usize);
			height = std::cmp::max(height, point.1 as usize);
		}

		let mut grid: Vec<Vec<bool>> = Vec::new();

		grid.resize(width +1, Vec::new());

		for col in grid.iter_mut() {
			col.resize(height +1, false);
		}

		for point in points {
			grid[point.0 as usize][point.1 as usize] = true;
		}

		return Self { grid }
	}


	fn width(&self) -> usize {
		return self.grid.len();
	}

	fn height(&self) -> usize {
		return self.grid[0].len();
	}

	fn num_points(&self) -> u16 {
		let mut count = 0;
		for x in &self.grid {
			for y in x {
				if *y == true { count += 1 }
			}
		}
		return count;
	}

	fn fold(&mut self, axis: &FoldAxis) {
		let merged_points = match axis {
			FoldAxis::Y(value) => self.fold_up(*value),
			FoldAxis::X(value) => self.fold_left(*value),
		};
	}


	fn fold_left(&mut self, fold_x:u16) {
		let row_start_idx = fold_x as usize +1;
		// move elements to the new location after fold
		for row_idx in row_start_idx..self.grid.len() {
			for col_idx in 0..self.grid[row_idx].len() {
				if self.grid[row_idx][col_idx] == false {continue}

				let new_row_idx =  2 * (fold_x as usize) - row_idx;
				let new_col_vec = &mut self.grid[new_row_idx];
				new_col_vec[col_idx] = true;
			}
		}
		// Resize grid
		self.grid.resize(fold_x as usize, Vec::new());
	}

	fn fold_up(&mut self, fold_y:u16) {
		let col_start_idx = fold_y as usize +1;
		for col_vec in self.grid.iter_mut() {
			for col_idx in col_start_idx..col_vec.len() {
				// no need to fold anything if there is no point
				if col_vec[col_idx] == false {continue}
				// fold the point around the Y coordinate (x coordinate changes)
				let new_col_idx = 2*(fold_y as usize) - col_idx;
				col_vec[new_col_idx] = true;
			}
			col_vec.resize(fold_y as usize, false);
		}
	}
}

impl fmt::Debug for Paper {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({},{}) - points: {}\n", self.width(), self.height(), self.num_points());
		for col_idx in 0..self.height() {
			for row_idx in 0..self.width() {

				let value = &self.grid[row_idx][col_idx];
				if *value == true {write!(f, "{}", '#')} else {write!(f, "{}", '.')};
			}
			write!(f, "{}", '\n');
		}

		write!(f, "{}", '\n')
	}
}

pub struct Input {
	points: Vec<Coord>,
	folds: Vec<FoldAxis>
}

#[aoc_generator(day13)]
pub fn parser(input: &str) -> Input{

	let mut input_split = input.split("\n\n");

	let points = input_split.next().unwrap().lines()
		.map(|point_line| {

			let mut numbers = point_line.split(',');
			let numbers = (
				numbers.next().unwrap().parse::<u16>().unwrap(),
				numbers.next().unwrap().parse::<u16>().unwrap()
			);

			return numbers;
		})
		.collect();


	let folds : Vec<FoldAxis> = input_split.next().unwrap().lines()
		.map(|fold_line| {
			let mut fold = fold_line.strip_prefix("fold along ").unwrap().split('=');
			return match fold.next().unwrap() {
				"x" => FoldAxis::X(fold.next().unwrap().parse::<u16>().unwrap()),
				"y" => FoldAxis::Y(fold.next().unwrap().parse::<u16>().unwrap()),
				_ => panic!(),
			}
		})
		.collect();

	return Input { points, folds};
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Input) -> u32 {
	let mut paper = Paper::init_with_points(&input.points);

	println!(" - ({},{})", paper.width(), paper.height());
	let first_fold = input.folds.first().unwrap();
	paper.fold(first_fold);
	return paper.num_points() as u32;
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Input) -> u16 {
	let mut paper = Paper::init_with_points(&input.points);
	println!(" - ({},{})", paper.width(), paper.height());
	for fold in &input.folds {
		paper.fold(fold);
	}
	println!("--------------------------------");
	println!("{:?}", paper);
	return paper.num_points();
}

mod tests {
	use super::*;

	const INPUT_LITERAL : &str =
"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

	#[test]
	fn test_day13_part1() {
		let input = parser(INPUT_LITERAL);
		let result = solve_part1(&input);
		assert_eq!(result, 17);
	}

	#[test]
	fn test_day13_part2() {
		let input = parser(INPUT_LITERAL);
		let result = solve_part2(&input);
		// assert_eq!(result, []);
	}

}