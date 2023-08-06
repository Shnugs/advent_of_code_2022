// --- Day 2: Rock Paper Scissors ---
// https://adventofcode.com/2022/day/2

// Play Rock, Paper, Scisssors. The rules are:
// - Rock beats Scissors
// - Scissors beats Paper
// - Paper beats Rock
// - If both players choose the same, it's a tie

// - Points are awarded as follows:
// - 6 point for a win
// - 3 points for a tie
// - 0 points for a loss
// - 1 point if rock is chosen
// - 2 points if paper is chosen
// - 3 points if scissors is chosen
// - ex.) If you win with scissors, you get 6 (for the win) + 3 (for choosing scissors) = 9 points

// Input: 
// A, X = Rock
// B, Y = Paper
// C, Z = Scissors

/// Reaturns parsed data from the input file that's ready to be consumed
/// # Returns
/// * input - A Vec of Vecs of Strings. Each Vec of Strings is a line from the input file.
fn read_input() -> Vec<Vec<String>> {
    let mut input: Vec<Vec<String>> = Vec::new();
    let file_path: &str = "src/input.txt";
    std::fs::read_to_string(&file_path).unwrap().split("\n").for_each(|line| {
        let mut line_vec: Vec<String> = Vec::new();
        line.split(" ").for_each(|word| {
            line_vec.push(word.to_string());
        });
        input.push(line_vec);
    });
    input
}

/// Get the points for a single game assuming
/// X, Y, Z are Rock, Paper, and Scissors respectively
/// # Arguments
/// * elf - The value that elf played 'A', 'B', or 'C'
/// * me - The value that I played 'X', 'Y', or 'Z'
/// # Returns
/// * Points won by 'me'
fn rock_paper_scissors(elf: String, me: String) -> i32 {
    match (elf.as_str(), me.as_str()) {
        // Win
        ("C", "X") => 6 + 1, // With Rock
        ("A", "Y") => 6 + 2, // With Paper
        ("B", "Z") => 6 + 3, // With Scissors
        // Lose
        ("A", "Z") => 0 + 3, // With Scissors
        ("B", "X") => 0 + 1, // With Rock
        ("C", "Y") => 0 + 2, // With Paper
        // Draw
        ("A", "X") => 3 + 1, // With Rock
        ("B", "Y") => 3 + 2, // With Paper
        ("C", "Z") => 3 + 3, // With Scissors
        // Default
        _          => 0
    }
}

/// Gets sum of scores for all games assuming 
/// 'X', 'Y', 'Z' are Rock, Paper, and Scissors respectively
/// # Argument
/// * input - A Vec of Vecs of Strings. 
/// Each Vec of Strings is a line from the input file.
fn part_one(input: &Vec<Vec<String>>) -> i32 {
    let mut score: i32 = 0;
    for line in input {
        score += rock_paper_scissors(line[0].to_string(), line[1].to_string());
    }
    score
}

/// Same as ```part_one()``` but uses map rather than for loop
/// # Argument
/// * input - A Vec of Vecs of Strings.
/// Each Vec of Strings is a line from the input file.
fn part_one_v2(input: &Vec<Vec<String>>) -> i32 {
    // do the functional equivalent of part one
    input.iter().map(|line| rock_paper_scissors(line[0].to_string(), line[1].to_string())).sum()
}

/// Returns points that would be awarded to me based on he value I have to play
///  to acheive the win_state
/// # Arguments
/// * win_state - The value that I have to play to win 'X', 'Y', or 'Z'
/// * elf - The value that elf played 'A', 'B', or 'C'
/// # Returns
/// Points that would be awarded for move selection to me based on
/// * 1.) The value that elf played and 
/// * 2.) The state the game should result in
///  
fn determine_move_points(match_outcome: String, elf_move: String) -> i32 {
    match match_outcome.as_str() {
        // Lose
        "X" => {
            match elf_move.as_str() {
                "A" => 3,
                "B" => 1,
                "C" => 2,
                _   => 0,
            }
        },
        // Draw
        "Y" => {
            match elf_move.as_str() {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _   => 0,
            }
        },
        // Win
        "Z" => {
            match elf_move.as_str() {
                "A" => 2,
                "B" => 3,
                "C" => 1,
                _   => 0,
            }
        },
        // Default
        _ => 0,
    }
}

/// Returns points awarded for a single game assuming "X", "Y", "Z" are 
/// "Lose", "Draw", and "Win" respectively
/// # Arguments
/// * elf - The value that elf played 'A', 'B', or 'C'
/// * me - The value that I played 'X', 'Y', or 'Z'
/// # Returns
/// * Points won by 'me'
fn lose_draw_win(elf: String, me: String) -> i32 {
    match me.as_str() {
        // "Lose",
        "X" => determine_move_points("X".to_string(), elf) + 0,
        // "Draw",
        "Y" => determine_move_points("Y".to_string(), elf) + 3,
        // "Win",
        "Z" => determine_move_points("Z".to_string(), elf) + 6,
        _   => 0,
    }
}

/// Gets sum of scores for all games assuming 
/// 'X', 'Y', 'Z' are Lose, Draw, and Win respectively
/// # Argument
/// * input - A Vec of Vecs of Strings. 
/// Each Vec of Strings is a line from the input file.
/// # Returns
/// * Sum of my scores for all games
fn part_two(input: &Vec<Vec<String>>) -> i32 {
    let mut score: i32 = 0;
    for line in input {
        score += lose_draw_win(line[0].to_string(), line[1].to_string());
    }
    score
}

/// Same as ```part_two()``` but uses map rather than for loop
/// # Argument
/// * input - A Vec of Vecs of Strings.
/// Each Vec of Strings is a line from the input file.
/// # Returns
/// * Sum of my scores for all games
fn part_two_v2(input: &Vec<Vec<String>>) -> i32 {
    // do the functional equivalent of part two
    input.iter().map(|line| lose_draw_win(line[0].to_string(), line[1].to_string())).sum()
}

fn main() {
    let input = read_input();

    println!("============");

    // Part 1
    let part_one_start = std::time::Instant::now();
    println!("part_1_v1: {}", part_one(&input));
    println!("Time: {}ms", part_one_start.elapsed().as_micros());

    println!("============");

    let part_one_v2_start = std::time::Instant::now();
    println!("part_1_v2: {}", part_one_v2(&input));
    println!("Time: {}ms", part_one_v2_start.elapsed().as_micros());

    println!("============");

    // Part 2
    let part_two_start = std::time::Instant::now();
    println!("part_2_v1: {}", part_two(&input));
    println!("Time: {}ms", part_two_start.elapsed().as_micros());

    println!("============");

    let part_two_v2_start = std::time::Instant::now();
    println!("part_2_v2: {}", part_two_v2(&input));
    println!("Time: {}ms", part_two_v2_start.elapsed().as_micros());

    println!("============");
}
