// https://adventofcode.com/2022/day/2

// --- Day 1: Calories ---
// In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: 
// they'd like to know how many Calories are being carried by the Elf carrying the most Calories. 
// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?

// This is a maximization problem. 
// The first task to overcome is parsing the input file.
// Then looping over items and getting sums of calories.
// Bonus points: See if I use more idiomatic Rust.

fn get_single_top_calories() -> i32 {
    let start = std::time::Instant::now();
    let mut max_calories = 0;
    // Get input from src/input.txt
    let input: String = std::fs::read_to_string("src/input.txt").unwrap();
    // break the input up by empty lines
    let elves: Vec<&str> = input.split("\n\n").collect();
    for food in elves {
        let meals: Vec<&str> = food.split("\n").collect();
        let mut elf_calories: i32 = 0;
        for meal in meals {
            elf_calories += meal.parse::<i32>().unwrap();
        }
        if elf_calories > max_calories {
            max_calories = elf_calories;
        }
    }
    println!("Time: {} micros", start.elapsed().as_micros());
    return max_calories;
}

fn get_three_top_calories() -> i32 {
    let start = std::time::Instant::now();
    let mut max_calories = vec![0; 3];
    // Get input from src/input.txt
    let input: String = std::fs::read_to_string("src/input.txt").unwrap();
    // break the input up by empty lines
    let elves: Vec<&str> = input.split("\n\n").collect();
    for food in elves {
        let meals: Vec<&str> = food.split("\n").collect();
        let mut elf_calories: i32 = 0;
        for meal in meals {
            elf_calories += meal.parse::<i32>().unwrap();
        }
        max_calories.push(elf_calories);
        max_calories.sort_by(|a, b| b.cmp(a));
        max_calories.pop();
    }
    println!("Time: {} micros", start.elapsed().as_micros());
    return max_calories.iter().sum();
}

fn main() {
    println!("Single Most Calories: {}", get_single_top_calories());
    println!("Three Most Calories: {}", get_three_top_calories());
}
