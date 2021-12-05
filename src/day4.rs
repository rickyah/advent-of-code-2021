use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::HashMap;

//  https://adventofcode.com/2021/day/4

type BoardIndex = u32;
type PositionInBoardIndex = u8;

pub struct Input {
	// Sequence of simulated randomly generated numbres
	drawn_numbers : Vec<u8>,
	// List of cardboards, each represented as a linear vector instead of a matrix
	cardboards : Vec<[u8;25]>,
	// Hashmap number => [ (board index, number position in board) ]
	// This hashmap will help us mark the boards faster, as for any drawn number
	// we'll have a fast look up to get the boards and positions where that 
	// number appeared
	numbers_in_boards : HashMap<u8, Vec<(BoardIndex, PositionInBoardIndex)>>
}

#[aoc_generator(day4)]
pub fn parser(input: &str) -> Input {
	
	let mut lines = input.lines();

	// Read list of drawn numbers
	let sequence = lines.next().unwrap()
		.split(',' )
		.map(|s| u8::from_str_radix(s, 10).unwrap())
		.collect();

	// Read card boards data and process numbers into a hashmap
	// We will generate the lookup hashmap here too 
	// (see Input::numbers_in_boards)
	let mut hmap = HashMap::new();

	// Save the numbers of all the cards read
	let mut cards : Vec<[u8;25]> = Vec::new();
	let mut number_idx_in_board : u8 = 0;
	let mut current_board : [u8;25] = [0;25];
	let mut current_board_idx : u32 = 0;

	for line in lines {
		// We'll count if we have read 25 numbers to
		//  determine that a board card is completed
		if line.is_empty() {continue;}
		
		// read the first 5 numbers
		let numbers_in_line : Vec<u8> = line
			.split_whitespace()
			.map(|s| u8::from_str_radix(s, 10).unwrap())
			.collect();

		assert!(numbers_in_line.len() == 5);

		for number in numbers_in_line {
			let entry = hmap.entry(number).or_insert(Vec::new());
			entry.push( (current_board_idx as u32, number_idx_in_board) );
			current_board[number_idx_in_board as usize] = number;
			number_idx_in_board += 1;
		}

		// We finished the board
		if number_idx_in_board == 25 {
			// The 0 represents a board with no numbers marked
			cards.push(current_board);

			current_board_idx += 1;
			number_idx_in_board = 0;
			current_board = [0;25];
			continue;
		}
	}

	return Input {
		cardboards: cards,
		drawn_numbers: sequence,
		numbers_in_boards: hmap
	};
}

// How many measurements are larger than the previous measurement?
#[aoc(day4, part1)]
pub fn solve_part1(input:&Input) -> u32 {
	let mut marks_in_boards = vec![0_u32; input.cardboards.len()];

	for (idx, num) in input.drawn_numbers.iter().enumerate() {
		// println!("idx: {}, num:{}", idx, num);
		let number_to_boards_map = input.numbers_in_boards.get(&num).unwrap();
		
		mark_number_in_boards(&mut marks_in_boards, number_to_boards_map);

		match find_first_winner_cardboard(&marks_in_boards) {
			Some(board_idx) => {

				let cardboard = &input.cardboards[board_idx];
				let cardboard_marks = marks_in_boards[board_idx];

				println!("board idx: {}, marks {:b}", board_idx, cardboard_marks);
				print_board(cardboard, cardboard_marks);

				let sum = sum_board_unmarked_positions(cardboard, cardboard_marks);
				let num = (*num as u32);
				let result : u32 = sum * num;
				return result;
			},
			None => continue
		}
	}
	return 0 
}

#[aoc(day4, part2)]
pub fn solve_part2(input:&Input) -> u32 {
	return 0;
}

pub fn print_board(board: &[u8;25], marks:u32) {
	for (idx, num) in board.iter().enumerate() {
		if idx % 5 == 0 { println!(""); }

		let num = *num as u32;
		let mask = 1_u32 << idx;
		if marks & mask != mask  {
			print!(" {} ", num);
		} else {
			print!("*{} ", num);
		}		
	}
	println!("");
}
pub fn sum_board_unmarked_positions(board: &[u8;25], marks:u32) -> u32 {	
	let mut sum = 0_u32;
	
	for (idx, num) in board.iter().enumerate() {
		let num = *num as u32;
		let mask = 1_u32 << idx;
		if marks & mask != mask { sum += num; }
	}
	return sum;
}
pub fn mark_number_in_boards(
	boards: &mut Vec<u32>, 
	positions: &Vec<(BoardIndex, PositionInBoardIndex)>) {

	for position in positions {
		let (board_idx, position_idx) = position;
		let board_idx = *board_idx as usize;
		let mask = 1_u32 << position_idx;
		boards[board_idx] = boards[board_idx] | mask;
	}
}


pub fn is_cardboard_winner(board_marks:u32) -> bool {
	const ROW_MASK : u32 = 0b00000000_00000000_00000000_00011111;
	const COL_MASK : u32 = 0b00000000_00010000_10000100_00100001;

	for idx in 0..5 {
		let row_mask = ROW_MASK << 5 * idx;
		let col_mask = COL_MASK << idx;

		if (board_marks & row_mask) == row_mask { return true; } 
		else if (board_marks & col_mask) == col_mask { return true; }
	}
	return false;
}

pub fn find_first_winner_cardboard(boards: &Vec<u32>) -> Option<usize> {
	const ROW_MASK : u32 = 0b00000000_00000000_00000000_00011111;
	const COL_MASK : u32 = 0b00001000_10000100_01000010_00100001;

	for (board_idx, board) in boards.iter().enumerate() {
		if is_cardboard_winner(*board) { return Option::Some(board_idx); }
	}
	return Option::None;
}


#[cfg(test)]
mod tests {
	use super::*;

const INPUT_LITERAL : &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

	#[test]
	fn test_day4_parse()
	{
		let input = parser(&INPUT_LITERAL);

		assert_eq!(input.drawn_numbers, vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1]);
		assert_eq!(input.cardboards.len(), 3);

		// Numbers for the test are in the range 0..=26,
		// for a total of 27 values
		assert_eq!(input.numbers_in_boards.len(), 27);
		
		let num_22 = input.numbers_in_boards.get(&22).unwrap();
		assert_eq!(num_22.len(), 3);
		assert_eq!(num_22[0], (0, 0));
		assert_eq!(num_22[1], (1, 4));
		assert_eq!(num_22[2], (2, 15));

		let num_26 = input.numbers_in_boards.get(&26).unwrap();
		assert_eq!(num_26.len(), 1);
		assert_eq!(num_26[0], (2, 13));
	}
	#[test]
	fn test_day4_part1() {
		let input = parser(&INPUT_LITERAL);
		let result = solve_part1(&input);
		assert_eq!(result, 4512);
	}

	#[test] 
	fn test_day4_part2() {
		let input = parser(&INPUT_LITERAL);
		let result = solve_part2(&input);
		assert_eq!(result, 1924);
	}
	
}