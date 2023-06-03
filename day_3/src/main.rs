use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    first_challenge_solution();
    second_challenge_solution();
}

fn first_challenge_solution() {
    let mut priority_sum: u32 = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_content) = line {
                // Note: This only works for standard characters which we are using in this
                // scenario. Non-unicode characters would cause an issue here.
                // Checkout "graphemes" for more details.
                let first_compartment: String =
                    line_content.chars().take(line_content.len() / 2).collect();
                let second_compartment: String =
                    line_content.chars().skip(line_content.len() / 2).collect();

                // Get distinct elements of each compartment
                let mut char_set_1 = HashSet::new();
                let mut char_set_2 = HashSet::new();
                for char in first_compartment.chars() {
                    char_set_1.insert(char);
                }
                for char in second_compartment.chars() {
                    char_set_2.insert(char);
                }

                // Find the element that exists in both HashSets and sum up its priority values
                for duplicate in char_set_1.intersection(&char_set_2) {
                    let ascii = *duplicate as u32;

                    // Get the priority of the character by using its ASCII value subtracted by its offset
                    if ascii > 90 {
                        priority_sum += ascii - 96;
                    } else {
                        priority_sum += ascii - 64 + 26;
                    }
                }
            }
        }
    }

    println!("The sum of all priorities is: {}", priority_sum);
}

fn second_challenge_solution() {
    let mut priority_sum: u32 = 0;
    let mut line_counter = 0;
    let mut first_line_set = HashSet::new();
    let mut second_line_set = HashSet::new();
    let mut third_line_set = HashSet::new();

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_content) = line {
                // Note: This only works for standard characters which we are using in this
                // scenario. Non-unicode characters would cause an issue here.
                // Checkout "graphemes" for more details.

                // Get distinct elements of each line
                if line_counter == 0 {
                    for char in line_content.chars() {
                        first_line_set.insert(char);
                    }
                    line_counter += 1;
                } else if line_counter == 1 {
                    for char in line_content.chars() {
                        second_line_set.insert(char);
                    }
                    line_counter += 1;
                } else {
                    for char in line_content.chars() {
                        third_line_set.insert(char);
                    }
                    line_counter = 0;
                }
            }

            if line_counter == 0 {
                // Find the element that exists in all HashSets and sum up its priority values
                let sets = vec![second_line_set.clone(), third_line_set.clone()];
                let mut intersection = first_line_set.clone();
                for set in sets {
                    intersection = intersection.intersection(&set).cloned().collect();
                }

                for duplicate in intersection {
                    let ascii = duplicate as u32;

                    // Get the priority of the character by using its ASCII value subtracted by its offset
                    if ascii > 90 {
                        priority_sum += ascii - 96;
                    } else {
                        priority_sum += ascii - 64 + 26;
                    }
                }

                // Reset the HashSets so they're ready to be used for the next group
                first_line_set.clear();
                second_line_set.clear();
                third_line_set.clear();
            }
        }
    }

    println!("The sum of all priorities is: {}", priority_sum);
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
