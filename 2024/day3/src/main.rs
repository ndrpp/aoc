use regex::Regex;
use std::fs;

fn main() {
    let content = fs::read_to_string("./input").expect("Failed to read file");
    let slice = content.as_str();

    let total = sum_multiplications(slice);

    println!("Total: {}", total)
}

fn sum_multiplications(input: &str) -> u32 {
    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let cmd_pattern = r"do\(\)|don't\(\)";
    let cmd_re = Regex::new(cmd_pattern).unwrap();
    let re = Regex::new(pattern).unwrap();
    let mut should_compute = true;

    let mut total = 0;
    let mut i = 0;

    while i < input.len() - 1 {
        let mut extra = 12;
        if i + 12 >= input.len() {
            extra = input.len() - i;
        }

        if let Some(mat) = cmd_re.find(&input[i..i + extra]) {
            i += mat.end();
            should_compute = mat.as_str() == "do()";
        } else if let Some(mat) = re.find(&input[i..i + extra]) {
            let matched_str = mat.as_str();

            if let Some(cap) = re.captures(matched_str) {
                let a: u32 = cap[1].parse().unwrap();
                let b: u32 = cap[2].parse().unwrap();

                if a <= 999 && b <= 999 {
                    if should_compute {
                        total += a * b;
                    }
                }
            }
            i += mat.end();
        } else {
            i += 1;
        }
    }

    total
}
