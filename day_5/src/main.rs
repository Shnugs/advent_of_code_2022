// --- Day 5: Supply Stacks ---
// https://adventofcode.com/2022/day/5

// --- Part One ---

// example of how columns are given
//     [D]    
// [N] [C]    
// [Z] [M] [P]
//  1   2   3 

// The above needs to translate into this
// [['N', 'Z'], ['D', 'C', 'M'], ['P']]

// example of how instructions are given
// move {number_of_items} from {origin_column} to {destination_column}
// move 1 from 2 to 1
// move 3 from 1 to 3
// move 2 from 2 to 1
// move 1 from 1 to 2

// Reads input file as a string
fn get_input_string_local() -> String {
    let file_path: &str = "src/input.txt";
    std::fs::read_to_string(file_path).unwrap()
}

fn rotate_string_clockwise(s: &str) -> String {    
    let s = s.trim_end_matches('\n');
    let mut result = String::new();
    let lines: Vec<&str> = s.split('\n').collect();

    let num_lines = lines.len();
    let num_chars = lines[0].len();

    // Rotate the string by iterating over the characters in each line
    // and appending them to the result in the correct order
    for i in 0..num_chars {
        for j in 0..num_lines {
            let c = lines[j].chars().nth(i);
            if let Some(ch) = c {
                result.push(ch);
            }
        }
    }
    result
}

fn parse_rotated_string(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_group = String::new();

    // Iterate over the characters in the string
    for c in s.chars() {
        // If the character is alphabetic, add it to the current group
        if c.is_alphabetic() {
            current_group.push(c);
        } else {
            // If the character is not alphabetic, add the current group to the
            // result and start a new group
            if !current_group.is_empty() {
                result.push(current_group);
                current_group = String::new();
            }
        }
    }
    result
}

fn parse_directions(s: &str) -> Vec<Vec<usize>> {
    s
    .split("\n")
    .map(|row| {
        let split: Vec<&str> = row.split(" ").collect();
        let num_items: usize = split[1].parse().unwrap();
        let origin: usize = split[3].parse().unwrap();
        let destination: usize = split[5].parse().unwrap();
        vec![num_items, origin, destination]
    })
    .collect()
}

fn do_part_one_directions(mut stacks: Vec<String>, directions: Vec<Vec<usize>>) -> Vec<String> {
    directions.iter().for_each(|direction|{
        let num_items = direction[0];
        let origin = direction[1];
        let destination = direction[2];
        let removed_chars: Vec<char> = stacks[origin - 1].drain(0..num_items).rev().collect();
        // println!("Move {} from {} to {}", removed_chars.iter().collect::<String>(), origin, destination);
        stacks[destination - 1].insert_str(0, &removed_chars.iter().collect::<String>());
        // println!("Resulting in: {:?}", stacks);
    });
    stacks
}

fn do_part_two_directions(mut stacks: Vec<String>, directions: Vec<Vec<usize>>) -> Vec<String> {
    directions.iter().for_each(|direction|{
        let num_items = direction[0];
        let origin = direction[1];
        let destination = direction[2];
        let removed_chars: Vec<char> = stacks[origin - 1].drain(0..num_items).collect();
        // println!("Move {} from {} to {}", removed_chars.iter().collect::<String>(), origin, destination);
        stacks[destination - 1].insert_str(0, &removed_chars.iter().collect::<String>());
        // println!("Resulting in: {:?}", stacks);
    });
    stacks
}

fn part_one(raw_input: &String) -> String {
    // Separate the stacks from the directions
    let split: Vec<&str> = raw_input.split("\n\n").collect();
    
    let raw_stacks: String = split[0].to_string();
    let raw_directions: String = split[1].to_string();

    let parsed_stacks: Vec<String> = parse_rotated_string(&rotate_string_clockwise(&raw_stacks));
    // println!("Parsed Stacks: {:?}", parsed_stacks);
    let parsed_directions: Vec<Vec<usize>> = parse_directions(&raw_directions);

    let result = do_part_one_directions(parsed_stacks, parsed_directions);
    // compose a string of the first character of each stack
    result.iter().map(|s| s.chars().nth(0).unwrap()).collect()
}

fn part_two(raw_input: &String) -> String {
    // Separate the stacks from the directions
    let split: Vec<&str> = raw_input.split("\n\n").collect();
    
    let raw_stacks: String = split[0].to_string();
    let raw_directions: String = split[1].to_string();

    let parsed_stacks: Vec<String> = parse_rotated_string(&rotate_string_clockwise(&raw_stacks));
    // println!("Parsed Stacks: {:?}", parsed_stacks);
    let parsed_directions: Vec<Vec<usize>> = parse_directions(&raw_directions);

    let result = do_part_two_directions(parsed_stacks, parsed_directions);
    // compose a string of the first character of each stack
    result.iter().map(|s| s.chars().nth(0).unwrap()).collect()
}

fn main() {
    let raw_input: String = get_input_string_local();
    
    // Part One
    println!("Part 1: {}", part_one(&raw_input));

    // Part Two
    println!("Part 2: {}", part_two(&raw_input));
}
