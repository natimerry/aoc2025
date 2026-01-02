
pub fn max_joltage(bank: &str) -> usize
{
    let mut max_val = 0u64;
    let mut max_first  = 0u64;

    for (i,ch) in bank.chars().enumerate()
    {
        let digit = ch.to_digit(10).unwrap() as u64;

        if i > 0{
            max_val = max_val.max(max_first*10 + digit);
        }
        max_first = max_first.max(digit);
    }

    max_val as usize

}
#[aoc(day3,part1)]
pub fn part1(input: &[&str]) -> usize
{
    let x: usize = input.iter().map(|bank| max_joltage(bank)).sum();

    
    x
}