use std::collections::HashMap;
use std::fs;

fn main() {
    let lines = read_lines("./input");
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    let mut total: i32 = 0;
    let mut similarity: i32 = 0;
    let mut appearances: HashMap<i32, bool> = HashMap::new();

    for line in lines.iter() {
        let values = line.split("   ").collect::<Vec<&str>>();
        left.push(values[0].parse().unwrap());
        appearances.insert(values[0].parse().unwrap(), true);

        right.push(values[1].parse().unwrap());
    }

    left.sort();
    right.sort();

    let mut i = 0;
    while i < left.len() {
        total += (left[i] - right[i]).abs();

        match appearances.get(&right[i]) {
            Some(_) => similarity += right[i],
            None => similarity += 0,
        }

        i += 1;
    }

    println!("Total: {total}");
    println!("Similiarity: {similarity}");
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
