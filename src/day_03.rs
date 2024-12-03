use std::{fs, env};
use regex::Regex;

fn part_1(input: &str) -> u64
{
    let mut sum: u64 = 0;

    for line in input.lines()
    {
        let re = Regex::new(r"mul\((?P<n1>\d{1,3}),(?P<n2>\d{1,3})\)").unwrap();

        for (_, [n1, n2]) in re.captures_iter(line).map(|c| c.extract())
        {
            sum += n1.parse::<u64>().unwrap() * n2.parse::<u64>().unwrap();
        }
    }

    sum
}

#[allow(dead_code,unused_imports, unused_variables)]
fn part_2(input: &str) -> u64
{
    0
}

pub fn main()
{
    let input_file = "input/day_03/input.txt";
    let input = fs::read_to_string(input_file);
    let part = env::args().nth(1).unwrap();

    match input
    {
        Ok(input) => 
        {
            if part == "1"
            {
                let result: u64 = part_1(&input);
                println!("{result}");
            }
            else if part == "2"
            {
                let result = part_2(&input);
                println!("{result}");
            }
        },
        Err(error) =>
        {
            panic!("{error}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input_file = "input/day_03/test.txt";
        let input = fs::read_to_string(input_file).unwrap();

        let result = part_1(&input);

        assert_eq!(result, 161);
    }
}