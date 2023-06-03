use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    first_challenge_solution();
    second_challenge_solution();
}

fn first_challenge_solution() {
    let mut score = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_content) = line {
                match line_content.as_str() {
                    // X => Rock (1)
                    // Y => Paper (2)
                    // Z => Scissors (3)
                    // Loose (0)
                    // Draw (3)
                    // Win (6)

                    // Enemy rock
                    "A X" => score += 3 + 1,
                    "A Y" => score += 6 + 2,
                    "A Z" => score += 0 + 3,
                    // Enemy paper
                    "B X" => score += 0 + 1,
                    "B Y" => score += 3 + 2,
                    "B Z" => score += 6 + 3,
                    // Enemy scissors
                    "C X" => score += 6 + 1,
                    "C Y" => score += 0 + 2,
                    "C Z" => score += 3 + 3,
                    _ => println!("Invalid input"),
                }
            }
        }
    }

    println!("My score is: {}", score);
}

fn second_challenge_solution() {
    let mut score = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_content) = line {
                match line_content.as_str() {
                    // X => Loose (0)
                    // Y => Draw (3)
                    // Z => Win (6)
                    // Rock (1)
                    // Paper (2)
                    // Scissors (3)

                    // Enemy rock
                    "A X" => score += 0 + 3,
                    "A Y" => score += 3 + 1,
                    "A Z" => score += 6 + 2,
                    // Enemy paper
                    "B X" => score += 0 + 1,
                    "B Y" => score += 3 + 2,
                    "B Z" => score += 6 + 3,
                    // Enemy scissors
                    "C X" => score += 0 + 2,
                    "C Y" => score += 3 + 3,
                    "C Z" => score += 6 + 1,
                    _ => println!("Invalid input"),
                }
            }
        }
    }

    println!("My score is: {}", score);
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
