use std::fs;
use regex::Regex;

#[derive(PartialEq, Debug)]
enum Level
{
    INCREASING,
    DECREASING,
    EQUAL
}

#[allow(dead_code)]
fn part_1(input: &str) -> u64
{
    let mut safe_lines = 0;
    let mut line_level: Level;
    // let mut vec: Vec<u64> = Vec::new();

    for line in input.lines()
    {
        // let re = Regex::new(r"^((\d+)\s+)+(\d+)$").unwrap();
        let re = Regex::new(r"(\d+)").unwrap();

        let matches: Vec<u64> = re.find_iter(line).map(|m| m.as_str().parse().unwrap()).collect();

        assert!(matches.len() > 1);

        if matches[0] < matches[1]
        {
            line_level = Level::INCREASING;
        }
        else if matches[0] > matches[1]
        {
            line_level = Level::DECREASING;
        }
        else
        {
            line_level = Level::EQUAL;
        }

        let mut unsafe_line: bool = false;
        if line_level == Level::INCREASING
        {
            for i in 0..(matches.len()-1)
            {
                if !(matches[i] < matches[i+1]) ||
                    (matches[i+1] - matches[i] > 3)
                {
                    unsafe_line = true;
                    break;
                }
            }
        }
        else if line_level == Level::DECREASING
        {
            for i in 0..(matches.len()-1)
            {
                if !(matches[i] > matches[i+1]) || 
                    (matches[i] - matches[i+1] > 3)
                {
                    unsafe_line = true;
                    break;
                }
            }
        }
        else
        {
            unsafe_line = true;
        }

        if !unsafe_line
        {
            safe_lines += 1;
        }
    }
    
    safe_lines
}

// fn part_2(input: &str) -> u64
// {
// }

pub fn main()
{
    let input_file = "input/day_02/input.txt";
    let input = fs::read_to_string(input_file);

    match input
    {
        Ok(input) => 
        {
            let result = part_1(&input);
            // let result = part_2(&input);
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
    fn test_1() {
        let input_file = "input/day_02/test.txt";
        let input = fs::read_to_string(input_file).unwrap();

        let result = part_1(&input);

        assert_eq!(result, 2);
    }
    
    // #[test]
    // fn test_2() {
    //     let input_file = "input/day_02/test.txt";
    //     let input = fs::read_to_string(input_file).unwrap();

    //     let result = part_2(&input);

    //     assert_eq!(result, 31);
    // }
}