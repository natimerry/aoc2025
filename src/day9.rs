use crate::utils::parser::coords_i64;

#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
    let mut max_area = 0;
    let coords = coords_i64(input);
    for (x,y) in &coords {
        for (i,j) in &coords{
            let area = ((x-i).abs() + 1)* ((y-j).abs() + 1);
            max_area = max_area.max(area);
        }
    }
    max_area as usize
}