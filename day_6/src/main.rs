// --- Day 6: Tuning Trouble ---
// https://adventofcode.com/2022/day/6

// The goal here is to find the index of the character that signals the start-of-packet-marker. 
// The start-of-packet-marker is the index of the last character in the first group of 4 unique characters.

fn get_input_string_local() -> String {
    let file_path: &str = "src/input.txt";
    std::fs::read_to_string(file_path).unwrap()
}

fn is_unique_chars(s: &str) -> bool {
    for c in s.chars() { 
        if s.matches(c).count() > 1 { 
            return false
        }
    };
    true
}

fn part_one(input: &str) -> usize {
    let mut index = 4;
    loop {
        if is_unique_chars(&input[(index - 4)..index]) {
            return index
        }
        index += 1;
    }
}

fn part_two(input: &str) -> usize {
    let mut index = 14;
    loop {
        if is_unique_chars(&input[(index - 14)..index]) {
            return index
        }
        index += 1;
    }
}

fn main() {
    let input = get_input_string_local();
    println!("Part 1 Index of signal start: {}", part_one(&input));

    println!("Part 2 Index of message start: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one_test_1() {
        assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    }
    #[test]
    fn part_one_test_2() {
        assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    }
    #[test]
    fn part_two_test_1() {
        assert_eq!(part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    }
    #[test]
    fn part_two_test_2() {
        assert_eq!(part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    }
}

// Learning
// ========
// I'm trying out tests for this one as having done so on day 5 would have 
// saved me a lot of time and pain
// - Export functions into the test module with 'use super::*;'

// You must explicitly use 'return' that you're returning a value from within a loop
