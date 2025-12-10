const INPUT_PATH: &str = "input.txt";

fn is_invalid(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();
    if len % 2 != 0 {
        return false;
    }
    let mid = len / 2;
    return id_str[..mid] == id_str[mid..];
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
