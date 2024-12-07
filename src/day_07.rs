use std::{collections::VecDeque, fs};
use regex::{Regex};

#[derive(Clone, Copy, Debug)]
enum Operators
{
    Add,
    Mul,
    NumOfOper
}

fn read_file(input: &str, matrix: &mut Vec<Vec<u64>>)
{
    for line in input.lines()
    {
        let re = Regex::new(r"(\d+)").unwrap();
        
        let number_vec: Vec<u64> = re.find_iter(line)
            .map(|c| c.as_str().parse::<u64>().unwrap())
            .collect();

        matrix.push(number_vec);
    }
    // dbg!(&matrix);
}

fn part_1(input: &str) -> u64
{
    let mut sum = 0;
    let mut matrix: Vec<Vec<u64>> = Vec::new();

    read_file(input, &mut matrix);

    for line in matrix
    {
        let mut options: VecDeque<Vec<Operators>> = VecDeque::new();
        let result = line[0];

        options.push_front(vec![Operators::Add;1]);
        options.push_front(vec![Operators::Mul;1]);

        // dbg!(&line);
        while options[0].len() < line.len()-2
        {
            let mut new_opt = vec![options.pop_front().unwrap();
                                                    Operators::NumOfOper as usize];
            
            new_opt[0].push(Operators::Add);
            new_opt[1].push(Operators::Mul);
            for i in 0..Operators::NumOfOper as usize
            {
                options.push_back(new_opt[i].clone());
            }
        }
        // dbg!(&options);

        let mut line_correct = false;
        
        for opt in options
        {
            let mut opt_result = line[1];
            
            for i in 0..opt.len()
            {
                match opt[i]
                {
                    Operators::Add =>
                    {
                        opt_result += line[i+2]
                    },
                    Operators::Mul =>
                    {
                        opt_result *= line[i+2];
                    },
                    _ => {}
                }
            }
            if result == opt_result
            {
                // dbg!(result);
                line_correct = true;
                break;
            }
        }

        if line_correct
        {
            sum += result
        }

    }
    sum
}

pub fn main()
{
    let input_file = "input/day_07/input.txt";
    // let input_file = "input/day_07/test_01.txt";
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
                // let result = part_2(&input);
                // println!("{result}");
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
        let input_file = "input/day_07/test_01.txt";
        let input = fs::read_to_string(input_file).unwrap();

        let result = part_1(&input);

        assert_eq!(result, 3749);
    }

}