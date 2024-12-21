use std::{fs, str};
use regex::Regex;

fn read_input(input: &str) -> ()
{
    let mut machines = Vec::new();

    let re = Regex::new(r".*X\+(\d+).*Y\+(\d+)\n.*X\+(\d+).*Y\+(\d+)\n.*X=(\d+).*Y=(\d+)\n").unwrap();

    let caps = re.captures_iter(input).map(|f| f.extract());

    for (_, [n0,n1,n2,n3,n4,n5]) in caps
    {
        let mut machine: Vec<u64> = Vec::new();
        machine.push((n0.parse::<u64>()).unwrap());
        machine.push((n1.parse::<u64>()).unwrap());
        machine.push((n2.parse::<u64>()).unwrap());
        machine.push((n3.parse::<u64>()).unwrap());
        machine.push((n4.parse::<u64>()).unwrap());
        machine.push((n5.parse::<u64>()).unwrap());
        dbg!(&machine);
        machines.push(machine);
    }
}

	
fn part_1(input: &str) -> usize
{
    let mut sum = 0;
    read_input(input);
    sum
}

#[allow(unused_variables,unused_mut)]
fn part_2(input: &str) -> usize
{
    let mut sum = 0;
    sum
}

pub fn main()
{
    // let input_file = "input/day_13/input.txt";
    let input_file = "input/day_13/test_01.txt";
    let input = fs::read_to_string(input_file);
    let part = "1";

    match input
    {
        Ok(input) => 
        {
            if part == "1"
            {
                let result  = part_1(&input);
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
    fn test_01() {
        let input_file = "input/day_13/test_01.txt";
        let input = fs::read_to_string(input_file).unwrap();

        let result = part_1(&input);

        assert_eq!(result, 0);
    }
}
