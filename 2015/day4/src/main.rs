use md5::compute;

fn main() {
    let mut i = 0;

    loop {
        let str = format!("yzbqklnj{}", i);
        let digest = compute(str.as_bytes());
        let hash_hex = format!("{:x}", digest);

        if hash_hex.starts_with("000000") {
            println!("index: {}", i);
            break;
        }
        i += 1;
    }
}
