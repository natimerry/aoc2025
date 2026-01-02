use crate::utils::math::wrap_add;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize  {
    let mut current_pos = 50;
    let mut hit_zero = 0;

    let instructions = input.split("\n").collect::<Vec<&str>>();    

    for int in instructions{
        let rotate_dir = match int.chars().nth(0).unwrap(){
            'L' => -1,
            'R' => 1,
            _ => unreachable!()
        };

        let amt: i64 = int[1..].parse().unwrap();
        current_pos = wrap_add(current_pos, amt*rotate_dir, 99);

        if current_pos == 0{
            hit_zero+=1;
        }
    }

    hit_zero as usize
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut pos: i64 = 50;
    let mut hits: i64 = 0;
    let n: i64 = 100;

    for line in input.lines() {
        let dir = line.as_bytes()[0];
        let amt: i64 = line[1..].parse().unwrap();

        let delta = if dir == b'R' { amt } else { -amt };
        let steps = delta.abs();

        // count hits during movement
        let first_hit = if delta > 0 {
            // moving right
            let fh = (n - pos) % n;
            if fh == 0 { n } else { fh }
        } else {
            // moving left
            if pos == 0 { n } else { pos }
        };

        if first_hit <= steps {
            hits += 1 + (steps - first_hit) / n;
        }

        // update position
        pos = (pos + delta).rem_euclid(n);
    }

    hits as usize
}