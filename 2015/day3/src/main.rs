use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let mut houses: HashMap<Vec<i16>, i16> = HashMap::new();
    houses.insert(vec![1000, 1000], 1);
    let mut x = 1000;
    let mut y = 1000;

    let content = read_to_string("./input").expect("Failed to read file");

    let mut i = 0;
    for char in content.chars() {
        if i % 2 == 0 {
            i += 1;
            continue;
        }
        match char {
            '>' => {
                x += 1;
            }
            '<' => {
                x -= 1;
            }
            'v' => {
                y -= 1;
            }
            '^' => {
                y += 1;
            }
            _ => continue,
        }

        match houses.get(&vec![x, y]) {
            Some(val) => houses.insert(vec![x, y], val + 1),
            None => houses.insert(vec![x, y], 1),
        };

        i += 1;
    }

    x = 1000;
    y = 1000;
    let mut j = 0;
    for char in content.chars() {
        if j % 2 != 0 {
            j += 1;
            continue;
        }
        match char {
            '>' => {
                x += 1;
            }
            '<' => {
                x -= 1;
            }
            'v' => {
                y -= 1;
            }
            '^' => {
                y += 1;
            }
            _ => continue,
        }

        match houses.get(&vec![x, y]) {
            Some(val) => houses.insert(vec![x, y], val + 1),
            None => houses.insert(vec![x, y], 1),
        };

        j += 1;
    }

    println!("Number of houses with at least 1 present: {}", houses.len());
}
