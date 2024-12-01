// use core::num;
use std::fs;
use regex::Regex;

fn return_sum(input: &str) -> u64
{
    let mut sum = 0;
    let mut vec_left: Vec<u64> = Vec::new();
    let mut vec_right: Vec<u64> = Vec::new();

    // dbg!(input);
    for line in input.lines()
    {
        let re = Regex::new(r"^(\d+)\s+(\d+)$").unwrap();
        for (_, [number_0, number_1]) in re.captures_iter(line).map(|c| c.extract())
        {
            vec_left.push(number_0.parse().unwrap());
            vec_right.push(number_1.parse().unwrap());
        }


    }

    vec_left.sort();
    vec_right.sort();

    assert_eq!(vec_left.len(), vec_right.len());

    for i in 0..vec_left.len()
    // for i in 0..10
    {
        let diff = u64::abs_diff(vec_left[i], vec_right[i]);
        sum += diff;   
    }

    sum
}

pub fn main()
{
    let input_file = "input/day_1_input.txt";
    let input = fs::read_to_string(input_file);

    match input
    {
        Ok(input) => 
        {
            let result = return_sum(&input);
            println!("{result}");
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
    fn example() {
        let input_file = "input/day_1_test.txt";
        let input = fs::read_to_string(input_file).unwrap();

        let result = return_sum(&input);

        assert_eq!(result, 11);
    }
}