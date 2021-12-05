use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::HashMap;

//  https://adventofcode.com/2021/day/4

type BoardIndex = u32;
type PositionInBoardIndex = u8;

pub struct Input {
	// Sequence of simulated randomly generated numbres
	drawn_numbers : Vec<u8>,
	// List of bingo gards, each represented as a linear vector
	// instead of a matrix
	bingo_cards : Vec<[u8;25]>,
	// Hashmap number => [ (board index, number position in board) ]
	// This hashmap will help us mark the boards faster, as for any drawn number
	// we'll have a fast look up to get the boards and positions where that 
	// number appeared
	numbers_in_cards : HashMap<u8, Vec<(BoardIndex, PositionInBoardIndex)>>
}

// Notes on marking the numbres in the bingo card
// ----------------------------------------------
// I choose to represent the action to mark a number in a bingo card using
// a u32 number, to allow checking if a card is completed faster.
// Each bit is a number position in the bingo board. I consider the position 0
// the value in a bingo card in top left, increasing from left to right, top to
// bottom:
//
//  0  1  2  3  4
//  5  6  7  8  9
// 10 11 12 13 14
// 15 16 17 18 19
// 20 21 22 23 24
//
// LSB in the mark represents position 0 of the bingo card, next LSB represents
// position 1, t c.
// A bingo board has 5x5=25 positions so last 7 MSB are unused.

// Notes on checking a completed bingo card
// -----------------------------------------
// We only need to check if a row or column is completely marked, we don't need
// the actual number values. As the marked numbers are represented by a u32 
// number, we can use bit masks to check it with just one comparison

// Mask for rows is: 
// 0b00000000_00000000_00000000_00011111 for row 0
// and we shift the bytes to the left by *five* for each row

// Mask for colums is: 
// 0b00000000_00010000_10000100_00100001 for column 0
// and we shift the bits to the left by *one* for each column


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
		if line.is_empty() { continue; }
		
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
		bingo_cards: cards,
		drawn_numbers: sequence,
		numbers_in_cards: hmap
	};
}

#[aoc(day4, part1)]
pub fn solve_part1(input:&Input) -> u32 {

	// We represents the marks in a board with a u32 number, where each bit
	// is a number position in the bingo board
	// See: fn mark_number_in_boards
	let mut marks_in_boards = vec![0_u32; input.bingo_cards.len()];

	for num in &input.drawn_numbers {
		let number_to_boards_map = input.numbers_in_cards.get(&num).unwrap();
		
		mark_number_in_boards(&mut marks_in_boards, number_to_boards_map);

		match find_first_completed_cardboard(&marks_in_boards) {
			Some(cardboard_idx) => {
				let cardboard = &input.bingo_cards[cardboard_idx];
				let cardboard_marks = marks_in_boards[cardboard_idx];

				let sum = sum_board_unmarked_numbers(cardboard, cardboard_marks);
				let num = *num as u32;
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
	let cardboards_len = input.bingo_cards.len();

	// We represents the marks in a board with a u32 number, where each bit
	// is a number position in the bingo board
	// See: fn mark_number_in_boards
	let mut marks_in_boards = vec![0_u32;cardboards_len];

	// We will use this vector to mark the index of cardboards as completed 
	// and avoid reprocessing
	let mut not_completed_cardboards_flags = vec![false;cardboards_len];

	for num in &input.drawn_numbers {
		let number_to_boards_map = input.numbers_in_cards.get(&num).unwrap();

		mark_number_in_boards(&mut marks_in_boards, number_to_boards_map);

		for cardboard_idx in 0..cardboards_len {
			// do not re-process cardboads marked as completed
			if not_completed_cardboards_flags[cardboard_idx] == true { 
				continue;
			}
	
			// Mark cardboard as winner
			if is_cardboard_completed(marks_in_boards[cardboard_idx]) { 
				not_completed_cardboards_flags[cardboard_idx] = true;
			}
			let completed_count = not_completed_cardboards_flags
				.iter()
				.filter(|x| **x == true)
				.count();

			// Check if it is the last cardboard
			if completed_count == cardboards_len {
				let cardboard = &input.bingo_cards[cardboard_idx];
				let cardboard_marks = marks_in_boards[cardboard_idx];

				let sum = sum_board_unmarked_numbers(cardboard, cardboard_marks);
				let num = *num as u32;
				let result : u32 = sum * num;
				return result;
			}
		}
	}

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
	println!("Marked positions mask: {:b}", marks);
	println!("");
}

pub fn sum_board_unmarked_numbers(board: &[u8;25], marks:u32) -> u32 {	
	let mut sum = 0_u32;
	
	for (idx, num) in board.iter().enumerate() {
		let num = *num as u32;
		let mask = 1_u32 << idx;
		if marks & mask != mask { sum += num; }
	}
	return sum;
}
 
pub fn mark_number_in_boards(
	bingo_card_marks: &mut Vec<u32>,
	number_positions_in_cards: &Vec<(BoardIndex, PositionInBoardIndex)>) {

	for position in number_positions_in_cards {
		let (board_idx, position_idx) = position;
		let board_idx = *board_idx as usize;
		let mask = 1_u32 << position_idx;
		bingo_card_marks[board_idx] = bingo_card_marks[board_idx] | mask;
	}
}

pub fn is_cardboard_completed(board_marks:u32) -> bool {
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

pub fn find_first_completed_cardboard(boards: &Vec<u32>) -> Option<usize> {
	for (board_idx, board) in boards.iter().enumerate() {
		if is_cardboard_completed(*board) { return Option::Some(board_idx); }
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
	fn test_day4_parse() {
		let input = parser(&INPUT_LITERAL);

		assert_eq!(input.drawn_numbers, vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1]);
		assert_eq!(input.bingo_cards.len(), 3);

		// Numbers for the test are in the range 0..=26,
		// for a total of 27 values
		assert_eq!(input.numbers_in_cards.len(), 27);
		
		let num_22 = input.numbers_in_cards.get(&22).unwrap();
		assert_eq!(num_22.len(), 3);
		assert_eq!(num_22[0], (0, 0));
		assert_eq!(num_22[1], (1, 4));
		assert_eq!(num_22[2], (2, 15));

		let num_26 = input.numbers_in_cards.get(&26).unwrap();
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

	#[test]
	fn test_winning_condition() {
		assert_eq!(is_cardboard_completed(0b11111110101001110101100), true);
	}
	
}
