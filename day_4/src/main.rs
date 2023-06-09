use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    check_for_overlapping();
}

fn check_for_overlapping() {
    let mut fully_contained = 0;
    let mut overlapping = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_content) = line {
                // Split the line at commas and dashes and convert each element into an integer
                let numbers: Vec<u16> = line_content
                    .split(&[',', '-'][..])
                    .map(|i| i.parse::<u16>().unwrap())
                    .collect();

                // Check if one pair is fully contained in the other one
                if (numbers[0] >= numbers[2] && numbers[1] <= numbers[3])
                    || (numbers[2] >= numbers[0] && numbers[3] <= numbers[1])
                {
                    fully_contained += 1;
                    overlapping += 1;
                // Check if one pair overlaps with the other one
                } else if cmp::max(numbers[0], numbers[2]) <= cmp::min(numbers[1], numbers[3]) {
                    overlapping += 1;
                }
            }
        }

        println!(
            "There are {} pairs fully contained in the other pair and {} pairs are overlapping.",
            fully_contained, overlapping
        );
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
