use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::HashMap;

// https://adventofcode.com/2021/day/6

pub struct Input {
	population: Vec<u8>
}


#[aoc_generator(day6)]
pub fn parser(input: &str) -> Input{
	let population = input.split(',')
		.map(|f| u8::from_str_radix(f, 10).unwrap())
		.collect();

	return Input{population: population};
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Input) -> u64 {
	return solver_optimized_hashmap(&input.population, 80);
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Input) -> u64 {
	return solver_optimized_hashmap(&input.population, 256);
}

pub fn solver_unoptimized(initial_population:&Vec<u8>, days:u16) -> u64{
	// I did not came out with this specific solution.
	// Mine was way more code, but similar in terms of speed and memory.
	// I came across this one while searching for help for the part 2, and
	// liked its simplicitly. 
	// https://www.youtube.com/watch?v=fHlWM8CIrlI

	// Explanation: each element in the vector is a fish, and the value is 
	// the days they have until they shouls create another fish.
	// Each day we iterate for all the population decreasing the count of 
	// days.
	// Whe decreasing the counter of fish with a value of 0, we instead
	// reset the counter back to 6 and add another fish with a value of 8.
	// After all the iterations, the size of the vector is the total population
	// generated.

	// This is not valid for part2, because for higher iteration numbers and due
	// to the nature of this exponential growth, you end up with a 
	// massive vector (hundreds of billions of entries) which is time cosuming to iterate, 
	// and also takes probably more memory than your system has
	// (1billion bytes == 1GB)

	// Bench data:
	// AOC 2021
	// Day 6 - Part 1 : 360268
	// generator: 20.3µs,
	// runner: 13.8707ms
	//
	// Day 6 - Part 2 : N/A

	let mut population = initial_population.clone();
	for _ in 0..days {
		let mut population_tmp : Vec<u8> = Vec::new();

		for fish in population {
			if fish == 0 {
				population_tmp.push(6);
				population_tmp.push(8);
			}
			else {
				population_tmp.push(fish-1);
			}
		}
		
		population = population_tmp;
	}

	return population.len() as u64;
}

pub fn solver_optimized(initial_population:&Vec<u8>, days: u16) -> u64 {

	// This was implemented after checking a proper solution. I did spent quite
	// some time thinking in a way to improve my original algorith, however the 
	// solution was more in the line of seen the problem with other eyes.

	// The idea is that tracking the number of fishes we count the fishes in the 
	// same state (the counter when the fish should spawn a new own)
	// Every fish in the same state behaves the same, so we can basically just treat
	// all the fishes in the same state as a whole. 
	// Each fish, then, belongs to a bucket: one for every possible count number
	// the fish can have (0..8, so 9 buckets in total)
	// Then, if we want to decrease the counter of the fishes that have 3 days 
	// to spawn new fishes, we just move the number of fishes from bucket 3 to 
	// bucket 2.
	// The exception is bucket 0, that requires to move the fishes to bucket 6
	// and add an equal amount of fishes to bucket 8 (the newly generated)
		
	// I've changed the solution to use a vector instead of a hashmap, gaining
	// some performance.

	// Bench data:
	// AOC 2021
	// Day 6 - Part 1 : 360268
	// 		generator: 34.6µs,
	// 		runner: 10.2µs
	
	// Day 6 - Part 2 : 1632146183902
	// 		generator: 15.3µs,
	// 		runner: 25.9µs
	
	let mut population : Vec<u64> = vec![0;9];
	
	for count in initial_population {
		population[*count as usize] += 1;
	}

	for _ in 0..days {
		
		// let mut population_tmp : HashMap<u8, u64> = HashMap::new();
		// for (idx, count) in &population  { population_tmp.insert(*idx, 0); }
		let mut population_tmp = vec![0;9];

		for (idx, count) in population.iter().enumerate() {
			if idx == 0 {
				population_tmp[6] += count;
				population_tmp[8] += count;
			}
			else {
				population_tmp[idx-1] += count;
			} 
		}

		population = population_tmp;
	}

	return population.iter().fold(0, |acc, v| acc + v) as u64;
}
	
pub fn solver_optimized_hashmap(initial_population:&Vec<u8>, days: u16) -> u64 {

	// same as solver_optimized but using a hashmap instead of a vector

	// Bench data:
	// AOC 2021
	// Day 6 - Part 1 : 360268
	// 		generator: 42.9µs,
	// 		runner: 188.8µs
	
	// Day 6 - Part 2 : 1632146183902
	// 		generator: 13.4µs,
	// 		runner: 240.7µs
	let mut population : HashMap<u8, u64> =  HashMap::new();
	
	for count in initial_population {
		let _ = *population.entry(*count).and_modify(|v| *v += 1).or_insert(1);
	}

	for _ in 0..days {
		
		let mut population_tmp : HashMap<u8, u64> = HashMap::new();
		for idx in population.keys() { population_tmp.insert(*idx, 0); }

		for (idx, count) in &population {
			if *idx == 0 {
				population_tmp.entry(6).and_modify(|v| *v += count).or_insert(*count);
				population_tmp.entry(8).and_modify(|v| *v += count).or_insert(*count);
			}
			else {
				population_tmp.entry(idx-1).and_modify(|v| *v += count).or_insert(*count);
			} 
		}

		population = population_tmp;
	}

	return population.values().fold(0, |acc, v| acc + v) as u64;
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT_LITERAL : &str = "3,4,3,1,2";
 

	#[test]
	fn test_day6_part1() {
		let input = parser(INPUT_LITERAL);
		let result = solver_optimized(&input.population, 80);
		assert_eq!(result, 5934);
	}

	#[test]
	fn test_day6_part2() {
		let input = parser(INPUT_LITERAL);
		let result = solver_optimized(&input.population, 256);
		assert_eq!(result, 26984457539);
	}
	
}
