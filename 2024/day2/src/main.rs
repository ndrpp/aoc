use std::fs;

enum Order {
    Asc,
    Desc,
}

const TOLERANCE: i32 = 3;

fn main() {
    let lines = read_lines("./input");
    let mut safe_lines: i32 = 0;

    for line in lines.iter() {
        let values = line.split(" ").collect::<Vec<&str>>();

        if is_safe(values) {
            safe_lines += 1;
        }
    }

    println!("Safe lines: {}", safe_lines)
}

fn is_safe(values: Vec<&str>) -> bool {
    let mut i = 0;
    let mut order = None;
    while i < values.len() {
        if i == values.len() - 1 {
            return true;
        }
        let cur: i32 = values[i].parse().unwrap();
        let next: i32 = values[i + 1].parse().unwrap();
        i += 1;

        let (new_order, success) = check_numbers(order, cur, next);
        order = new_order;
        if !success {
            break;
        }
    }

    return false;
}

fn check_numbers(input_order: Option<Order>, first: i32, second: i32) -> (Option<Order>, bool) {
    let order: Order;
    match input_order {
        Some(val) => order = val,
        None => {
            if first < second {
                order = Order::Asc;
            } else {
                order = Order::Desc;
            }
        }
    }

    match order {
        Order::Asc => (
            Some(Order::Asc),
            !(second <= first || second > first + TOLERANCE),
        ),
        Order::Desc => (
            Some(Order::Desc),
            !(second >= first || second < first - TOLERANCE),
        ),
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
