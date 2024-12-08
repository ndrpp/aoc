use std::fs;

enum Order {
    Asc,
    Desc
}

fn main() {
    let lines = read_lines("./input");
    let mut safe_lines: i32 = 0;

    for line in lines.iter() {
        let order: Order;
        let values = line.split(" ").collect::<Vec<&str>>();
        if values[0] > values[1] {
            order = Order::Desc
        } else {
            order = Order::Asc
        }
    
        let mut i = 0;
        while i < values.len() {
            if i + 1 == values.len() {
                safe_lines += 1;
                break;
            }
            let cur: i32 = values[i].parse().unwrap();
            let next: i32 = values[i + 1].parse().unwrap();
            match order {
                Order::Asc => {
                    if next - cur < 1 || next - cur > 3 {
                        break;
                    }
                },
                Order::Desc => {
                    if cur - next > 3 || cur - next < 1 {
                        break;
                    }
                }
            }
            
            i += 1;
        }
    }

    println!("Safe lines: {}", safe_lines)
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
