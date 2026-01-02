fn input_ranges(list: Vec<&str>) -> Vec<(i64,i64)>
{
    list.iter().map(|x|{
        let (start,end) = x.split_once("-").unwrap();
        let start: i64 = start.parse().unwrap();
        let end: i64 = end.parse().unwrap();
        (start,end)
    }).collect()
}


fn is_repetitive(num: &str) -> Option<usize> {
    let k = num.len();
    
    for l in 2..=k {
        let pattern_len = k / l;
        
        if k % l == 0 && pattern_len > 0 {
            let pattern = &num[..pattern_len];
            
            // Check if repeating the pattern l times gives us the original string
            if pattern.repeat(l) == num {
                return Some(l);
            }
        }
    }
    None
}

#[aoc(day2,part1)]
pub fn part1(input: &str) -> i64 {
    let lines: Vec<&str> = input.split(',').collect();
    let ranges = input_ranges(lines);
    
    let mut p1 = 0i64;
    
    for (start, end) in ranges {
        for num in start..=end {
            let s = num.to_string();
            
            if let Some(l) = is_repetitive(&s) {
                if l == 2 {
                    p1 += num;
                }
            }
        }
    }
    
    p1
}
#[aoc(day2,part2)]
pub fn part2(input: &str) -> i64 {
    let lines: Vec<&str> = input.split(',').collect();
    let ranges = input_ranges(lines);
    
    let mut p2 = 0i64;
    
    for (start, end) in ranges {
        for num in start..=end {
            let s = num.to_string();
            
            if is_repetitive(&s).is_some() {
                p2 += num;
            }
        }
    }
    
    p2
}