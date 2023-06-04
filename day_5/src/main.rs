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
        let mut crates: [Vec<char>; 9] = [
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];

        for line in lines {
            if let Ok(line_content) = line {
                // Hard-coded way of determining in which line we are
                // < 8 => crates
                // > 9 => operations

                // Get initial crate configuration
                if line_counter < 8 {
                    for (i, c) in line_content.chars().into_iter().enumerate() {
                        // Only get the characters at the location where characters can appear and
                        // check if it's a character (!= whitespace).
                        if [1, 5, 9, 13, 17, 21, 25, 29, 33].contains(&i) && !c.is_whitespace() {
                            crates[(i - 1) / 4].push(c);
                        }
                    }

                // Get operations
                } else if line_counter > 9 {
                    // Get the numbers and filter out the operations
                    let operations: Vec<u8> = line_content
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
                        .collect();
                    println!("{:?}", operations);
                }
            }

            line_counter += 1;
        }
        println!("crates: {:?}", crates);
    }
}

fn second_challenge_solution() {
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line_content) = line {}
        }
    }
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
