const INPUT_PATH: &str = "input.txt";

fn main() {
    let input: String = std::fs::read_to_string(INPUT_PATH).unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let grid_size = lines.len();
    let mut roll_grid: Vec<Vec<bool>> = Vec::new();
    for line in lines {
        roll_grid.push(line.chars().map(|c| c == '@').collect::<Vec<bool>>());
    }
    let mut accessible: u32 = 0;
    for x in 0 .. grid_size {
        for y in 0 .. grid_size {
            if !roll_grid[x][y] {
                continue;
            }
            let mut neighbours: u8 = 0;
            for (dx, dy) in [
                (1, 0), (-1, 0), (0, 1), (0, -1),
                (1, 1), (1, -1), (-1, 1), (-1, -1),
            ] {
                let neighbour_x = x as isize + dx;
                let neighbour_y = y as isize + dy;
                if neighbour_x < 0 ||
                    neighbour_y < 0 ||
                    neighbour_x >= grid_size as isize ||
                    neighbour_y >= grid_size as isize {
                    continue;
                }
                neighbours += roll_grid[neighbour_x as usize][neighbour_y as usize] as u8;
            }
            if neighbours < 4 {
                accessible += 1;
            }
        }
    }
    println!("{}", accessible);
}
