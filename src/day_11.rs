use std::fs;

fn read_input(input: &str) -> Vec<u64>
{
    let vec = input.split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect();

    vec
}

fn get_digits(n: u64) -> u32
{
    let mut div = 10;
    let mut digits = 1;

    while n / div != 0
    {
        div *= 10;
        digits += 1;
    }

    digits
}
	
fn part_1(input: &str) -> u64
{
    let iters = 25;
    let mut vec = read_input(input);

    for _iter in 0..iters
    {
        let mut i = 0;
        while i < vec.len()
        {
            let n = vec[i];
            let digits = get_digits(n);
            if n == 0
            {
                vec[i] = 1;
            }
            else if digits % 2 == 0
            {
                let right = n % 10u64.pow(digits/2);
                let left = n / 10u64.pow(digits/2);
                vec[i] = left;
                vec.insert(i+1, right);
                i += 1;     // skip new inserted number
            }
            else
            {
                vec[i] *= 2024;
            }
            i += 1;
        }
            // dbg!(&vec);
    }

    vec.len() as u64
}

#[allow(unused_variables,unused_mut)]
fn part_2(input: &str) -> u64
{
    let mut sum = 0;
    sum
}

pub fn main()
{
    let input_file = "input/day_11/input.txt";
    // let input_file = "input/day_11/test_01.txt";
    let input = fs::read_to_string(input_file);
    let part = "1";

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
    fn test_01() {
        let input_file = "input/day_11/test_01.txt";
        let input = fs::read_to_string(input_file).unwrap();

        let result = part_1(&input);

        assert_eq!(result, 55312);
    }
}
