const INPUT_PATH: &str = "input.txt";

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Endpoint {
    Start,
    End,
}

fn main() {
    let input: String = std::fs::read_to_string(INPUT_PATH).unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut endpoints: Vec<(u64, Endpoint)> = Vec::new();
    for line in lines {
        if !line.contains('-') {
            continue;
        }
        let (start, end): (&str, &str) = line.split_once('-').unwrap();
        endpoints.push((start.parse::<u64>().unwrap(), Endpoint::Start));
        endpoints.push((end.parse::<u64>().unwrap(), Endpoint::End));
    }
    endpoints.sort();
    let mut coverage: u16 = 0;
    let mut n_fresh: u64 = 0;
    let mut range_start: u64 = 0;
    for endpoint in endpoints {
        match endpoint {
            (id, Endpoint::Start) => {
                if coverage == 0 {
                    range_start = id;
                }
                coverage += 1;
            }
            (id, Endpoint::End) => {
                coverage -= 1;
                if coverage == 0 {
                    n_fresh += id - range_start + 1;
                }
            }
        }
    }
    println!("{}", n_fresh);
}
