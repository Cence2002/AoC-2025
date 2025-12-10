const INPUT_PATH: &str = "input.txt";

fn main() {
    let input: String = std::fs::read_to_string(INPUT_PATH).unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut total_joltage: u32 = 0;
    for line in lines {
        let bank: Vec<u8> = line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        let mut max_joltage: u8 = 0;
        for i in 0..bank.len() {
            for j in i+1..bank.len() {
                let joltage = bank[i] * 10 + bank[j];
                max_joltage = max_joltage.max(joltage as u8);
            }
        }
        total_joltage += max_joltage as u32;
    }
    println!("{}", total_joltage);
}
