fn input_ranges(list: Vec<&str>) -> Vec<(i64,i64)>
{
    list.iter().map(|x|{
        let (start,end) = x.split_once("-").unwrap();
        let start: i64 = start.parse().unwrap();
        let end: i64 = end.parse().unwrap();
        (start,end)
    }).collect()
}


#[aoc(day5,part1)]
fn part1(input: &str) -> usize
{
    let (ranges,id_para) = input.split_once("\n\n").unwrap();

    let ranges = input_ranges(ranges.split("\n").collect::<Vec<&str>>());
    let id_list = id_para.split("\n").map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let mut fresh = 0;
    for id in id_list{
        for (lo,hi) in &ranges{
            if *lo <= id && id <= *hi {
                fresh+=1;
                break;
            }
        }
    }
    
    fresh as usize
}

#[aoc(day5,part2)]
fn part2(input: &str) -> usize
{
    let (ranges,_id_para) = input.split_once("\n\n").unwrap();

    let mut ranges = input_ranges(ranges.split("\n").collect::<Vec<&str>>());

    let mut merged: Vec<(i64, i64)> = Vec::with_capacity(ranges.len());

    ranges.sort_by_key(|&(start, _)| start);

        for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    let mut sum = 0;
    for (lo,hi) in merged{
        sum+= (hi - lo) + 1;
    }
    sum as usize
}