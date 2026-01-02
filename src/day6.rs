
#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let rows: Vec<Vec<&str>> = input
        .lines()
        .map(|row| row.split(' ').filter(|s| !s.is_empty()).collect())
        .collect();


    let ops = &rows[rows.len() - 1];
    let num_rows = &rows[..rows.len() - 1];

    let mut sum = 0i64;

    for i in 0..ops.len() {
        let op = ops[i];

        let numbers: Vec<i64> = num_rows
            .iter()
            .filter_map(|row| row.get(i)?.parse().ok())
            .collect();

        let result = match op {
            "*" => numbers.iter().product(),
            "+" => numbers.iter().sum(),
            _ => 0,
        };

        sum += result;
    }

    sum as usize
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid.iter().map(|r| r.len()).max().unwrap();

    let mut sum = 0i64;
    let mut col = cols as isize - 1;

    while col >= 0 {
        // skip empty separator columns
        while col >= 0 && (0..rows).all(|r| *grid[r].get(col as usize).unwrap_or(&' ') == ' ') {
            col -= 1;
        }
        if col < 0 {
            break;
        }

        let end = col as usize;

        // find left boundary of this problem
        while col >= 0 && (0..rows).any(|r| *grid[r].get(col as usize).unwrap_or(&' ') != ' ') {
            col -= 1;
        }

        let start = (col + 1) as usize;

        // operator is bottom non-space char
        let op = (start..=end)
            .find_map(|c| {
                let ch = *grid[rows - 1].get(c).unwrap_or(&' ');
                if ch == '+' || ch == '*' { Some(ch) } else { None }
            })
            .unwrap();

        let mut numbers = Vec::new();

        for c in start..=end {
            let mut value = 0i64;
            let mut place = 1;

            for r in (0..rows - 1).rev() {
                if let Some(d) = grid[r].get(c).and_then(|x| x.to_digit(10)) {
                    value += (d as i64) * place;
                    place *= 10;
                }
            }

            if place > 1 {
                numbers.push(value);
            }
        }

        let result = match op {
            '+' => numbers.iter().sum(),
            '*' => numbers.iter().product(),
            _ => 0,
        };

        sum += result;
    }


    sum as usize
}