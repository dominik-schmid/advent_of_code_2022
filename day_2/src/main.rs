use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut score = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_content) = line {
                match line_content.as_str() {
                    "A X" => score += 4,
                    "A Y" => score += 8,
                    "A Z" => score += 3,
                    "B X" => score += 1,
                    "B Y" => score += 5,
                    "B Z" => score += 9,
                    "C X" => score += 7,
                    "C Y" => score += 2,
                    "C Z" => score += 6,
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
