use std::collections::HashMap;

use crate::utils::parser::grid_chars;

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = grid_chars(input);

    let mut takyon_beam: HashMap<usize, Vec<usize>> = HashMap::new(); // stores idx and where takyon beam will start going down from

    let mut count = 0;
    for i in 0..grid.len() {
        let current_beams = takyon_beam.get(&i).cloned().unwrap_or_default();

        for j in 0..grid[i].len() {
            // get which j indexes takyon beam is supposed to be in right now and compare if we are at spliter
            let ch = grid[i][j];

            if ch == 'S' {
                // we start the tachyon here
                takyon_beam.entry(i + 1).or_default().push(j);
                continue;
            }

            if ch == '^' {
                if current_beams.contains(&j) {
                    // this means we have hit a splitter

                    let x = takyon_beam.entry(i + 1).or_default();
                    x.push(j - 1);
                    x.push(j + 1);
                    count += 1;
                }
                continue;
            }

            if current_beams.contains(&j) {
                takyon_beam.entry(i + 1).or_default().push(j);
            }
        }
    }

    count as usize
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = grid_chars(input);

   let mut timelines: HashMap<usize, HashMap<usize, u64>> = HashMap::new();

    // get S and initialize the first beam
    for (r, row) in grid.iter().enumerate() {
        if let Some(c) = row.iter().position(|&ch| ch == 'S') {
            timelines.entry(r + 1).or_default().insert(c, 1);
            break; 
        }
    }

    // 2. Process row by row
    for i in 0..grid.len() {
        if let Some(current_beams) = timelines.get(&i).cloned() {
            for (j, count) in current_beams {
                let ch = grid.get(i).and_then(|row| row.get(j)).unwrap_or(&'.');
                match ch {
                    '^' => {
                        // we split it left and right both
                        let next_row = timelines.entry(i + 1).or_default();
                        if j > 0 { 
                            *next_row.entry(j - 1).or_insert(0) += count; 
                        }
                        *next_row.entry(j + 1).or_insert(0) += count;
                    },
                    _ => {
                        // just pass through, there cant be any qyantum shit
                        *timelines.entry(i + 1).or_default().entry(j).or_insert(0) += count;
                    }
                }
            }
        }
    }

    // Sum all timelines 
   let x = timelines.get(&grid.len())
        .map(|row| row.values().sum())
        .unwrap_or(0);
     
     x as usize
}
