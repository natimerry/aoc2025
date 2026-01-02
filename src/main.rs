use std::{env, time::Instant};


// fn main() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() != 3 {
//         eprintln!("usage: aoc <year> <day>");
//         std::process::exit(1);
//     }

//     let year: u32 = args[1].parse().expect("invalid year");
//     let day: usize = args[2].parse::<usize>().expect("invalid day") - 1;

//     let day_idx: usize = args[2].parse::<usize>().expect("invalid day") - 1;

//     let days = years::get_year(year).expect("unknown year");
//     let day = days.get(day_idx).expect("day not implemented");

//     let input = read_input(&format!("inputs/{}/day{:02}.txt", year, day_idx + 1));

//     let t = Instant::now();
//     let p1 = (day.part1)(&input);
//     let t1 = t.elapsed();
//     println!("part 1: {}\n\ttook ({:?})", p1, t1);

//     let t = Instant::now();

//     let p2 = (day.part2)(&input);
//     let t2 = t.elapsed();

//     println!("part 2: {}\n\ttook ({:?})", p2, t2);
// }

fn main() -> eyre::Result<()> {
    aoc::run(advent_of_code::register::register_runners)
}