use std::ops::Add;
use crate::utils::parser::coords_i64;

#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
    let mut max_area = 0;
    let coords = coords_i64(input);
    for i in 0..coords.len() {
        let (x1,y1) = coords[i];

        for j in i+1..coords.len(){
            let (x2,y2) = coords[j];
            let area = (x1 - x2).abs().add(1) * (y1 - y2).abs().add(1);
            max_area = max_area.max(area);
        }
    }
    max_area as usize
}