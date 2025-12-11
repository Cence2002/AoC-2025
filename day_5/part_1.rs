const INPUT_PATH: &str = "input.txt";

fn main() {
    let input: String = std::fs::read_to_string(INPUT_PATH).unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut id_ranges: Vec<std::ops::RangeInclusive<u64>> = Vec::new();
    let mut n_fresh: u16 = 0;
    for line in lines {
        if line == "" {
            continue;
        }
        if line.contains('-') {
            let (start, end): (&str, &str) = line.split_once('-').unwrap();
            id_ranges.push(start.parse::<u64>().unwrap() ..= end.parse::<u64>().unwrap());
            continue;
        }
        let id: u64 = line.parse::<u64>().unwrap();
        n_fresh += id_ranges.iter().any(|id_range| id_range.contains(&id)) as u16;
    }
    println!("{}", n_fresh);
}
