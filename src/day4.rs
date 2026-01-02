use crate::utils::parser::grid_chars;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let grid = grid_chars(input);
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1), // up-left, up, up-right
        (0, -1),
        (0, 1), // left, right
        (1, -1),
        (1, 0),
        (1, 1), // down-left, down, down-right
    ];

    let mut access = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '.' {
                continue;
            }

            let mut count = 0;

            for (di, dj) in directions {
                let ni = i as isize + di;
                let nj = j as isize + dj;

                if ni >= 0
                    && ni < grid.len() as isize
                    && nj >= 0
                    && nj < grid[i].len() as isize
                    && grid[ni as usize][nj as usize] == '@'
                {
                    count += 1;
                }
            }

            if count < 4 {
                access += 1;
            }
        }
    }
    access as usize
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let mut grid = grid_chars(input);
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut total_removed = 0;
    let mut changed = true;

    while changed {
        changed = false;
        let mut to_remove = Vec::new();

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] != '@' {
                    continue;
                }

                let mut count = 0;
                for (di, dj) in directions {
                    let ni = i as isize + di;
                    let nj = j as isize + dj;

                    if ni >= 0
                        && ni < grid.len() as isize
                        && nj >= 0
                        && nj < grid[i].len() as isize
                        && grid[ni as usize][nj as usize] == '@'
                    {
                        count += 1;
                    }
                }

                if count < 4 {
                    to_remove.push((i, j));
                }
            }
        }

        if !to_remove.is_empty() {
            changed = true;
            total_removed += to_remove.len();

            for (i, j) in to_remove {
                grid[i][j] = '.';
            }
        }
    }

    total_removed
}
