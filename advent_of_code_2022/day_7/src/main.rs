// --- Day 7: No Space Left On Device ---
// https://adventofcode.com/2022/day/7

// --- Part One ---
// INPUT
// A '\n' separated list command line IO
// They come in 4 flavors:
// 1.) $ cd /         -- Command: change directory
// 2.) $ ls           -- Command: list directory
// 3.) dir a          -- Directory name
// 4.) 14848514 b.txt -- File Size/File name

// From this, a file tree can be derived

// OUTPUT
// the sum of all directories whose sum file size is <= 100_000
// Note that commands always start with a '$'

// Steps to solve:
// 1. Read in the input
// 2. Parse the input into a tree (hard part)
// 3. Traverse the tree and sum the file sizes (directories size sums include sub directory sizes)
// 4. Determine which directories have a size <= 100_000
// 5. Print the sum of those directories

// 

use std::collections::HashMap;

#[derive(Debug)]
enum Node {
    File { name: String, size: u64 },
    Directory { name: String, children: HashMap<String, Node> },
}

impl Node {
    fn new_directory(name: &str) -> Node {
        Node::Directory {
            name: name.to_string(),
            children: HashMap::new(),
        }
    }

    fn new_file(name: &str, size: u64) -> Node {
        Node::File {
            name: name.to_string(),
            size,
        }
    }
}

fn parse_input(input: &str) -> Node {
    let mut lines = input.lines();
    let mut root = Node::new_directory("/");
    let mut current_node = &mut root;

    while let Some(line) = lines.next() {
        let parts: Vec<&str> = line.split(' ').collect();
        match parts[0] {
            "cd" => {
                let new_node = current_node
                    .children
                    .get_mut(parts[1])
                    .expect("Directory does not exist");
                current_node = match new_node {
                    Node::Directory { children, .. } => children,
                    _ => panic!("Cannot cd into a file"),
                };
            }
            "ls" => {
                let children = current_node
                    .children
                    .iter()
                    .map(|(name, node)| match node {
                        Node::Directory { .. } => format!("dir {}", name),
                        Node::File { size, .. } => format!("{} {}", size, name),
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                println!("{}", children);
            }
            _ => {
                let name = parts[1];
                let size = parts[0].parse::<u64>().expect("Invalid file size");
                let node = Node::new_file(name, size);
                current_node.children.insert(name.to_string(), node);
            }
        }
    }

    root
}

fn main() {
    let input = "cd /\nls\ndir a\n14848514 b.txt\n";
    let tree = parse_input(input);
    println!("{:#?}", tree);
}


// Learning comments:
// This seems like it might be a good place to learn about structs. 