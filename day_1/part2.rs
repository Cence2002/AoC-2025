fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut position: i32 = 50;
    let mut password: u32 = 0;
    for line in lines {
        let sign = line.chars().next();
        let distance = &line[1..].parse::<u32>().unwrap();
        for _ in 0..*distance {
            match sign {
                Some('R') => position += 1,
                Some('L') => position -= 1,
                _ => panic!("wtf"),
            }
            position = position.rem_euclid(100);
            if position == 0 {
                password += 1;
            }
        }
    }
    println!("{}", password);
}