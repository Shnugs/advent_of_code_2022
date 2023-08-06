// --- Day 3: Rucksack Reorganization ---
// https://adventofcode.com/2022/day/3

// Input will be lines of the form:
// vJrwpWtwJgWrhcsFMMfFFhFp
// Split the line in half to represent two compartments like so:
// vJrwpWtwJgWr hcsFMMfFFhFp
// The goal is to find the single character that 
// is present in both compartments
// In this case, "P"

// Values for answers are a-Z 1-52 
use std::collections::HashMap;

/// # Returns
/// * HashMap of characters and their assigned values
fn generate_score_hash() -> HashMap<char, i32> {
    let mut pairs: HashMap<char, i32> = HashMap::new();
    let mut score: i32 = 1;
    // Generate scores for a-z, 1-26
    (97..123).for_each(|i| {
        pairs.insert(i as u8 as char, score);
        score += 1;
    });
    // Generate scores for A-Z, 27-52
    (65..91).for_each(|i| {
        pairs.insert(i as u8 as char, score);
        score += 1;
    });
    pairs
}


fn get_input_string() -> String {
    let file_path: &str = "src/input.txt";
    std::fs::read_to_string(file_path).unwrap()
}

/// Takes the string from input.txt and prepares it for part one consumption
/// # Arguments
/// * `input` - String from input.txt
/// # Returns
/// * parsed_input - A Vec of a Vec of Strings representing the two compartments
fn parse_part_one_input(input: &String) -> Vec<Vec<String>> {
    let mut parsed_input: Vec<Vec<String>> = Vec::new();
    input.split("\n").for_each(|line| {
        let mut line_vec: Vec<String> = Vec::new();
        let first_half: String = line.chars().take(line.len() / 2).collect();
        let second_half: String = line.chars().skip(line.len() / 2).collect();
        line_vec.push(first_half);
        line_vec.push(second_half);
        parsed_input.push(line_vec);
    });
    parsed_input
}

/// Takes the string from input.txt and prepares it for part two consumption
/// # Arguments
/// * `input` - String from input.txt
/// # Returns
/// * parsed_input - A Vec of a Vec of Strings representing three elves' rucksacks
fn parse_part_two_input(input: &String) -> Vec<Vec<String>> {
    let mut parsed_input: Vec<Vec<String>> = Vec::new();
    // break input into lines
    // group lines into 3s
    input.split('\n').collect::<Vec<&str>>().chunks(3).for_each(|chunk| {
        let mut line_vec: Vec<String> = Vec::new();
        chunk.iter().for_each(|line| {
            line_vec.push(line.to_string());
        });
        parsed_input.push(line_vec);
    });
    parsed_input
}

/// Find the character that is present in both compartments
/// # Arguments
/// * compartments - A Vec of Strings. Each String is a compartment
/// # Returns
/// * common_char - The character that is present in both compartments
fn find_common_char_part_one(compartments: &Vec<String>) -> char {
    let mut common_char: char = ' ';
    let compartment_one_chars: Vec<char> = compartments[0].chars().collect();
    let compartment_two_chars: Vec<char> = compartments[1].chars().collect();
    compartment_one_chars.iter().for_each(|c| {
        if compartment_two_chars.contains(c) {
            common_char = *c;
        }
    });
    common_char
}

/// Find the character that is present in all three elves' compartments
/// # Arguments
/// * elves - A Vec of Strings. Each String is an elf's rucksack
/// # Returns
/// * common_char - The character that is present in all three elves' compartments
fn find_common_char_part_two(elves: &Vec<String>) -> char {
    let mut common_char: char = ' ';
    let elf_one_chars: Vec<char> = elves[0].chars().collect();
    let elf_two_chars: Vec<char> = elves[1].chars().collect();
    let elf_three_chars: Vec<char> = elves[2].chars().collect();
    elf_one_chars.iter().for_each(|c| {
        if elf_two_chars.contains(c) && elf_three_chars.contains(c) {
            common_char = *c;
        }
    });
    common_char
}

fn do_part_one(input: &String, score_hash: &HashMap<char, i32>) -> i32 {
    let mut part_one_total: i32 = 0;
    parse_part_one_input(&input).iter().for_each(|rucksack| {
        let common_char: char = find_common_char_part_one(rucksack);
        part_one_total += score_hash.get(&common_char).unwrap();
    });
    part_one_total
}

fn do_part_two(input: &String, score_hash: &HashMap<char, i32>) -> i32 {
    let mut part_two_total: i32 = 0;
    parse_part_two_input(&input).iter().for_each(|rucksack| {
        let common_char: char = find_common_char_part_two(rucksack);
        part_two_total += score_hash.get(&common_char).unwrap();
    });
    part_two_total
}

fn main() {
    // Generate elemts used by both parts
    let score_hash: HashMap<char, i32> = generate_score_hash();
    let input: String = get_input_string();

    // time part one
    let start_one = std::time::Instant::now();
    let part_one_score = do_part_one(&input, &score_hash);
    let duration_one = start_one.elapsed().as_micros();

    // time part two
    let start_two = std::time::Instant::now();
    let part_two_score = do_part_two(&input, &score_hash);
    let duration_two = start_two.elapsed().as_micros();


    println!("Part One Total: {} - in {}micros", part_one_score, duration_one);
    println!("Part Two Total: {} - in {}micros", part_two_score, duration_two);
}


// TIL:
// 1.) The Rust type 'HashMap' 
// 2.) Generating key/value pairs functionally turns out to be ~5% faster 
//     than imperatively in this case
// 3.) 
