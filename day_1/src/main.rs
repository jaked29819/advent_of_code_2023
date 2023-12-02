use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    let mut sum = 0;
    let re = Regex::new(r"[0-9]{1}").unwrap();

    if let Ok(lines) = read_line("./input.txt") {
        for line in lines {
            if let Ok(text) = line {
                // added for part 2
                let text = text
                    .replace("one", "o1e")
                    .replace("two", "t2o")
                    .replace("three", "t3e")
                    .replace("four", "f4r")
                    .replace("five", "f5e")
                    .replace("six", "s6x")
                    .replace("seven", "s7n")
                    .replace("eight", "e8t")
                    .replace("nine", "n9e");

                let numbers : Vec<_> = re.find_iter(&text)
                                            .map(|m| m.as_str())
                                            .collect();

                let first_number: i32 = numbers[0].parse().unwrap();
                let last_number: i32 = numbers.last().unwrap().parse().unwrap();

                sum += (first_number * 10) + last_number;
            }
        }
    }

    println!("Sum: {}", sum);
}

fn read_line<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
