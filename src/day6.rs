use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::VecDeque;
use std::collections::HashMap;

//  https://adventofcode.com/2021/day/6

pub struct Input {
	start_days_until_spawn: Vec<u8>
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub struct FishGrowthModelInput {
	days_until_spawn: u8,
	start_day: u16, 
}

#[aoc_generator(day6)]
pub fn parser(input: &str) -> Input{
	let population = input.split(',')
		.map(|f| u8::from_str_radix(f, 10).unwrap())
		.collect();

	return Input{start_days_until_spawn: population};
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Input) -> u64 {
	let mut input_population : Vec<FishGrowthModelInput> = Vec::new();

	for days_until_spawn in &input.start_days_until_spawn {
		input_population.push(FishGrowthModelInput{
			days_until_spawn: *days_until_spawn,
			start_day: 0		});
	}
	
	let result = fish_growth_simulator(input_population, 80_u16);
	return result;
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Input) -> u64 {
	let mut input_population : Vec<FishGrowthModelInput> = Vec::new();

	for days_until_spawn in &input.start_days_until_spawn {
		input_population.push(FishGrowthModelInput{
			days_until_spawn: *days_until_spawn,
			start_day: 0});
	}
	
	let result = fish_growth_simulator(input_population, 256_u16);
	return result;
}

fn fish_growth_simulator(initial_population: Vec<FishGrowthModelInput>, days: u16) -> u64 {
	let mut fishes_to_simulate : VecDeque<FishGrowthModelInput> = VecDeque::new();

	for fish in initial_population {
		fishes_to_simulate.push_back(fish);
	}
	
	
	let result = simulate_fish_growth(&mut fishes_to_simulate, days);
	
	// let result = simulate_fish_growth_uncached(&mut fishes_to_simulate, days);
	
	return result;
}

fn simulate_fish_growth(fishes: &mut VecDeque<FishGrowthModelInput>, total_days: u16) -> u64 {
	let mut fish_count =  0_u64;

	let mut cache : HashMap<FishGrowthModelInput, Vec<FishGrowthModelInput>> = HashMap::new();

	while !fishes.is_empty() {
		if let Some(fish) = fishes.pop_front() {
			
			if let Some(generated_fishes) = cache.get(&fish) {
				let new_fishes = generated_fishes;

				fish_count += 1;

				for new_fish in new_fishes {
					fishes.push_back(*new_fish);
				}

			} else {
				let new_fishes = simulate_new_fishes_from(fish, total_days);

				fish_count += 1;

				for new_fish in &new_fishes {
					fishes.push_back(new_fish.clone());
				}

				cache.insert(fish, new_fishes);
			}

		}
	};

	return fish_count;
}

fn simulate_fish_growth_uncached(fishes: &mut VecDeque<FishGrowthModelInput> , days: u16) -> u64 {
	let mut fish_count =  0_u64;

	while !fishes.is_empty() {
		if let Some(fish) = fishes.pop_front() {
			let new_fishes = simulate_new_fishes_from(fish, days);
			
			fish_count += 1;

			for new_fish in new_fishes {
				fishes.push_back(new_fish);
			}
		}
	};

	return fish_count;
}


fn simulate_new_fishes_from(fish:FishGrowthModelInput, max_day: u16) ->  Vec<FishGrowthModelInput> {
	let mut current_day = fish.start_day;
	let mut days_until_spawn = fish.days_until_spawn ; //as i16;
	let mut new_fishes : Vec<FishGrowthModelInput> = Vec::new();

	while current_day <= max_day {
		
		if days_until_spawn == 0 {
			days_until_spawn = 6; 
			if current_day + 1 <= max_day {	
				new_fishes.push(FishGrowthModelInput{
					days_until_spawn: 8,
					start_day: current_day + 1
				});
			}
		} else {
			days_until_spawn -= 1;
		}
		
		current_day += 1;
	}

	return new_fishes;
}


#[cfg(test)]
mod tests {
	use super::*;

	const INPUT_LITERAL : &str = "3,4,3,1,2";
 
	#[test]
	fn test_1() {
		let fish = FishGrowthModelInput{
			days_until_spawn: 3,
			start_day: 0
		};

		let result = simulate_new_fishes_from(fish, 18);

		assert_eq!(result.len(), 3);
		assert_eq!(result[0], FishGrowthModelInput{ days_until_spawn: 8, start_day: 4});
		assert_eq!(result[1], FishGrowthModelInput{ days_until_spawn: 8, start_day: 11} );
		assert_eq!(result[2], FishGrowthModelInput{ days_until_spawn: 8, start_day: 18} );
		
	}
	#[test]
	fn test_2() {
		let mut fishes_to_simulate : VecDeque<FishGrowthModelInput> = VecDeque::new();

		fishes_to_simulate.push_back(FishGrowthModelInput{
			days_until_spawn: 3,
			start_day: 0
		});

		fishes_to_simulate.push_back(FishGrowthModelInput{
			days_until_spawn: 4,
			start_day: 0
		});

		fishes_to_simulate.push_back(FishGrowthModelInput{
			days_until_spawn: 3,
			start_day: 0
		});

		fishes_to_simulate.push_back(FishGrowthModelInput{
			days_until_spawn: 1,
			start_day: 0
		});

		fishes_to_simulate.push_back(FishGrowthModelInput{
			days_until_spawn: 2,
			start_day: 0
		});
		
		let days = 18_u16;
		let result = simulate_fish_growth(&mut fishes_to_simulate, days);

		assert_eq!(result, 26);
	}
	
	#[test]
	fn test_3() {
		let mut fishes_to_simulate : VecDeque<FishGrowthModelInput> = VecDeque::new();

		fishes_to_simulate.push_back(FishGrowthModelInput{
			days_until_spawn: 3,
			start_day: 0
		});

		fishes_to_simulate.push_back(FishGrowthModelInput{
			days_until_spawn: 4,
			start_day: 0
		});
		let days = 18_u16;
		let result = simulate_fish_growth(&mut fishes_to_simulate, days);

		assert_eq!(result, 9);
	}
	// #[test]
	fn test_day6_part1() {
		let input = parser(INPUT_LITERAL);
		let result = solve_part1(&input);
		assert_eq!(result, 5934);
	}

	#[test]
	fn test_day6_part2() {
		let input = parser(INPUT_LITERAL);
		let result = solve_part2(&input);
		assert_eq!(result, 26984457539);
	}


	#[test]
	fn test_cache() {
		let mut cache : HashMap<FishGrowthModelInput, u64> = HashMap::new();
		
		cache.insert(FishGrowthModelInput{days_until_spawn: 3, start_day: 8}, 3);

		assert_eq!(cache.contains_key(&FishGrowthModelInput{days_until_spawn: 3, start_day: 8}), true);
		assert_eq!(cache.contains_key(&FishGrowthModelInput{days_until_spawn: 2, start_day: 8}), false);
		assert_eq!(cache.contains_key(&FishGrowthModelInput{days_until_spawn: 3, start_day: 1}), false);
	}

	
}