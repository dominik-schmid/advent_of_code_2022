use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut current_sum = 0;
    let mut biggest_sum = 0;
    let mut second_biggest_sum = 0;
    let mut third_biggest_sum = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_content) = line {
                if line_content == "" {
                    // Update the three biggest sums if a bigger sum has been found
                    if current_sum > biggest_sum {
                        third_biggest_sum = second_biggest_sum;
                        second_biggest_sum = biggest_sum;
                        biggest_sum = current_sum;
                    } else if current_sum > second_biggest_sum {
                        third_biggest_sum = second_biggest_sum;
                        second_biggest_sum = current_sum;
                    } else if current_sum > third_biggest_sum {
                        third_biggest_sum = current_sum;
                    }
                    current_sum = 0;
                } else {
                    current_sum += line_content.parse::<i32>().unwrap_or(0);
                }
            }
        }
    }

    println!("Biggest sum is: {}", biggest_sum);
    println!(
        "Sum of the three biggest sums is: {}",
        biggest_sum + second_biggest_sum + third_biggest_sum
    );
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
