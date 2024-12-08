use std::fs;

fn main() {
    let mut total_surface: i32 = 0;
    let mut total_ribbon: i32 = 0;
    let lines = read_lines("./input");

    for line in lines.iter() {
        let values = line.split("x").collect::<Vec<&str>>();

        let length: i32 = values[0].parse().expect("Failed to parse length");
        let width: i32 = values[1].parse().expect("Failed to parse width");
        let height: i32 = values[2].parse().expect("Failed to parse height");

        total_surface += compute_surface(length, width, height);
        total_ribbon += compute_ribbon(length, width, height);
    }

    println!("Total surface: {total_surface}");
    println!("Total ribbon: {total_ribbon}");
}

fn compute_ribbon(l: i32, w: i32, h: i32) -> i32 {
    let mut vec = vec![l, w, h];
    vec.sort();

    2 * vec[0] + 2 * vec[1] + l * w * h
}

fn compute_surface(l: i32, w: i32, h: i32) -> i32 {
    let area = 2 * l * w + 2 * w * h + 2 * h * l;

    let mut vec = vec![l, w, h];
    vec.sort();

    area + vec[0] * vec[1]
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
