
pub fn max_joltage(bank: &str, n: usize) -> usize
{
    let m = bank.len();
    let mut to_rem = m -n;

    let mut stack: Vec<char> = Vec::with_capacity(n);

    for ch in bank.chars(){
        while !stack.is_empty() && to_rem > 0 && stack.last().unwrap() < &ch
        {
            stack.pop();
            to_rem-=1;
        }

        stack.push(ch);
    }

    stack.iter().collect::<String>().parse().unwrap()
}
#[aoc(day3,part1)]
pub fn part1(input: &[&str]) -> usize
{
    let x: usize = input.iter().map(|bank| max_joltage(bank,2)).sum();

    
    x
}