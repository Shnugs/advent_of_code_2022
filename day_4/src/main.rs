// --- Day 4: Camp Cleanup ---
// https://adventofcode.com/2022/day/4

// Basically there is a camp cleanup list where elves are assigned cleanup in 
// sections. The elves compare sections they're respnosible for in pairs in 
// the form of [A-B,X-Y]

// Part 1: Count the instances where one section entirely contains the other

// Rather than copying the input into a file, I'm trying a url import
// Use reqwest to get the text from https://adventofcode.com/2022/day/4/input and return it as a String
// use ureq::{Agent, AgentBuilder};
// use std::time::Duration;

// fn get_input_string_http() -> Result<(), ureq::Error> {
//     let agent: Agent = ureq::AgentBuilder::new()
//         .timeout_read(Duration::from_secs(5))
//         .timeout_write(Duration::from_secs(5))
//         .build();
//     let body: String = agent.get("https://adventofcode.com/2022/day/4/input")
//         .call()?
//         .into_string()?;
//     println!("{}", body);
//     Ok(())
// }

// Readys input file as a string
fn get_input_string_local() -> String {
    let file_path: &str = "src/input.txt";
    std::fs::read_to_string(file_path).unwrap()
}

// 13-53,17-82
// 32-32,32-42
// [[[13,53],[17,82]],[[32,32],[32,42]]]
// Parse the input string into a vector of vectors of vectors of integers
fn parse_part_one(input_string: &String) -> Vec<Vec<Vec<i32>>> {
    input_string.split("\n").map(|line|
        line
        .split(",")
        .map(|elf|
            elf
            .split("-")
            .map(|x| x
                .parse::<i32>()
                .unwrap()
            ).collect()
        ).collect()
    ).collect()
}

// Counts the number of ranges that are entirely contained within another
fn part_one(input: Vec<Vec<Vec<i32>>>) -> i32 {
    input.iter().filter(|elves|  
        if (elves[0][0] >= elves[1][0] && elves[0][1] <= elves[1][1]) || 
        (elves[1][0] >= elves[0][0] && elves[1][1] <= elves[0][1]) {
            true
        }
        else {false}
    ).count() as i32
}

// Counts the number of ranges that intersect
fn part_two(input: Vec<Vec<Vec<i32>>>) -> i32 {
    input.iter().filter(|elves| {
        let elf_one_range: Vec<i32> = (elves[0][0]..=elves[0][1]).collect();
        let elf_two_range: Vec<i32> = (elves[1][0]..=elves[1][1]).collect();
        if elf_one_range.contains(&elves[1][0]) || 
        elf_one_range.contains(&elves[1][1]) || 
        elf_two_range.contains(&elves[0][0]) || 
        elf_two_range.contains(&elves[0][1]) 
        {true}
        else 
        {false}
    }).count() as i32
}

fn main() {
    // print out get_input_string()
    let input = get_input_string_local();

    // Part 1
    let part_one: i32 = part_one(parse_part_one(&input));
    println!("Part One - Total Overlap: {:?}", part_one);

    // Part 2
    let part_two: i32 = part_two(parse_part_one(&input));
    println!("Part Two - Partial Overlap: {:?}", part_two);
}
