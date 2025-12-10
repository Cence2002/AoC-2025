const INPUT_PATH: &str = "input.txt";
const MAX_BATTERIES: u8 = 12;

fn main() {
    let input: String = std::fs::read_to_string(INPUT_PATH).unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut total_joltage: u64 = 0;
    for line in lines {
        let bank: Vec<u8> = line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        let bank_size = bank.len();
        let mut max_joltage: Vec<Vec<Vec<u64>>> = vec![vec![vec![0; MAX_BATTERIES as usize + 1]; bank_size + 1]; bank_size + 1];
        for batteries in 1 ..= MAX_BATTERIES as usize {
            for chunk_length in batteries ..= bank_size as usize {
                for chunk_start in 0 ..= bank_size - chunk_length {
                    let chunk_end = chunk_start + chunk_length;
                    if batteries == 0 {
                        continue;
                    }
                    if batteries == 1 && chunk_length == 1 {
                        max_joltage[chunk_start][chunk_end][batteries] = bank[chunk_start] as u64;
                        continue;
                    }
                    for prefix_batteries in 0 ..= batteries {
                        let suffix_batteries = batteries - prefix_batteries;
                        for prefix_length in prefix_batteries ..= chunk_length - suffix_batteries {
                            let breakpoint = chunk_start + prefix_length;
                            let prefix_max_joltage = max_joltage[chunk_start][breakpoint][prefix_batteries];
                            let suffix_max_joltage = max_joltage[breakpoint][chunk_end][suffix_batteries];
                            let joltage = prefix_max_joltage * 10_u64.pow(suffix_batteries as u32) + suffix_max_joltage;
                            max_joltage[chunk_start][chunk_end][batteries] = max_joltage[chunk_start][chunk_end][batteries].max(joltage);
                        }
                    }
                }
            }
        }
        total_joltage += max_joltage[0][bank_size][MAX_BATTERIES as usize];
    }
    println!("{}", total_joltage);
}