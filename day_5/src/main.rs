use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    first_challenge_solution();
    second_challenge_solution();
}

fn first_challenge_solution() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut line_counter = 0;
        // VecDeques are used because during creation we want to add the characters at the front
        let mut crates: [VecDeque<char>; 9] = Default::default();

        for line in lines {
            if let Ok(line_content) = line {
                // Hard-coded way of determining in which line we are
                // < 8 => crates
                // > 9 => operations

                // Get initial crate configuration
                if line_counter < 8 {
                    get_crates(&line_content, &mut crates);
                } else if line_counter > 9 {
                    let operations = get_operations(&line_content);

                    // Move number of crates (operations[0]) from source (operations[1] - 1) to target (operations[2] - 1)
                    for _ in 0..operations[0] {
                        let moved_element = crates[operations[1] as usize - 1].pop_back().unwrap();
                        crates[operations[2] as usize - 1].push_back(moved_element);
                    }
                }
            }

            line_counter += 1;
        }

        let mut solution: String = String::from("");
        for i in 0..crates.len() {
            solution.push(*crates[i].back().unwrap());
        }
        println!("Solution of first challenge: {}", solution);
    }
}

fn second_challenge_solution() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut line_counter = 0;
        // VecDeques are used because during creation we want to add the characters at the front
        let mut crates: [VecDeque<char>; 9] = Default::default();

        for line in lines {
            if let Ok(line_content) = line {
                // Hard-coded way of determining in which line we are
                // < 8 => crates
                // > 9 => operations

                if line_counter < 8 {
                    get_crates(&line_content, &mut crates);
                } else if line_counter > 9 {
                    let operations = get_operations(&line_content);

                    // Move number of crates (operations[0]) from source (operations[1] - 1) to target (operations[2] - 1)
                    let split_at_index =
                        crates[operations[1] as usize - 1].len() - operations[0] as usize;
                    let mut moved_elements =
                        crates[operations[1] as usize - 1].split_off(split_at_index);
                    crates[operations[2] as usize - 1].append(&mut moved_elements);
                }
            }

            line_counter += 1;
        }

        let mut solution: String = String::from("");
        for i in 0..crates.len() {
            solution.push(*crates[i].back().unwrap());
        }
        println!("Solution of second challenge: {}", solution);
    }
}

fn get_crates(line_content: &str, crates: &mut [VecDeque<char>]) {
    for (i, c) in line_content.chars().into_iter().enumerate() {
        // Only get the characters at the location where characters can appear and
        // check if it's a character (!= whitespace).
        if [1, 5, 9, 13, 17, 21, 25, 29, 33].contains(&i) && !c.is_whitespace() {
            // Add new character at the front of the vector so we can get the last
            // element of it, like in a stack, later.
            crates[(i - 1) / 4].push_front(c);
        }
    }
}

fn get_operations(line_content: &str) -> Vec<u8> {
    // Get the numbers and filter out the operations
    line_content
        .split(' ')
        .enumerate()
        .filter_map(|(i, n)| {
            // Filter out the operations, i.e. every element with an even index
            if i % 2 == 0 {
                None
            } else {
                Some(n.parse::<u8>().unwrap())
            }
        })
        .collect()
}

// Source: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
