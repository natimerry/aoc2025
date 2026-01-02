pub fn ints(input: &str) -> Vec<i64> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<i64>().unwrap())
        .collect()
}

pub fn grid_chars(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect()
}

pub fn coords_i64(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(',').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

pub fn blocks(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}
