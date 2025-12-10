const INPUT_PATH: &str = "input.txt";

fn is_invalid(id: u64) -> bool {
    let id = id.to_string();
    let len = id.len();
    for k in 1..=(len-1) {
        if len % k != 0 {
            continue;
        }
        let chunk = &id[..k];
        let mut invalid = true;
        for i in (0..len).step_by(k) {
            if &id[i..i+k] != chunk {
                invalid = false;
                break;
            }
        }
        if invalid {
            return true;
        }
    }
    return false;
}

fn main() {
    let input: String = std::fs::read_to_string(INPUT_PATH).unwrap();
    let ranges: Vec<&str> = input.split(",").collect();
    let mut invalid: Vec<u64> = Vec::new();
    for range in ranges {
        let (start, end): (&str, &str) = range.split_once("-").unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();
        for id in start..=end {
            if is_invalid(id) {
                invalid.push(id);
            }
        }
    }
    println!("{}", invalid.iter().sum::<u64>());
}
