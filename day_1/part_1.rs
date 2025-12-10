fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut position: i32 = 50;
    let mut password: u32 = 0;
    for line in lines {
        let sign: char = line.chars().next().unwrap();
        let distance: i32 = line[1..].parse::<i32>().unwrap();
        position += if sign == 'R' { distance } else { -distance };
        position = position.rem_euclid(100);
        if position == 0 {
            password += 1;
        }
    }
    println!("{}", password);
}
