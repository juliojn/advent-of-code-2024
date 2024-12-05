use std::{fs};
use regex::Regex;


fn read_file(input: &str, v0: &mut Vec<u64>, v1: &mut Vec<u64>, updates: &mut Vec<Vec<u64>>)
{
    for line in input.lines()
    {
        let re = Regex::new(r"(\d+)\|(\d+)").unwrap();
        
        match re.captures(line)
        {
            Some(caps) =>
            {
                v0.push(caps.get(1).unwrap().as_str()
                        .parse::<u64>().unwrap());
                v1.push(caps.get(2).unwrap().as_str()
                        .parse::<u64>().unwrap());
            }
            None =>
            {
                let re = Regex::new(r"\d+").unwrap();
                let update: Vec<u64> = re.find_iter(line)
                    .map(|c| c.as_str().parse::<u64>().unwrap())
                    .collect();
                
                if !update.is_empty()
                {
                    updates.push(update);
                }
            }
        }


    }

    assert_eq!(v0.len(), v1.len())
}

fn part_1(input: &str) -> u64
{
    let mut sum = 0;
    let mut v0 = Vec::new();
    let mut v1 = Vec::new();
    let mut updates = Vec::new();

    read_file(input, &mut v0, &mut v1, &mut updates);

    // dbg!(&v0, &v1, &updates);

    for update_line in updates
    {
        // dbg!(&update_line);
        let mut valid_update = true;

        for i in 0..v0.len()
        {
            for upd_idx_0 in 0..update_line.len()
            {
                if v0[i] == update_line[upd_idx_0] && valid_update
                {
                    for upd_idx_1 in 0..update_line.len()
                    {
                        if v1[i] == update_line[upd_idx_1] && valid_update
                        {
                            if upd_idx_0 > upd_idx_1
                            {
                                // dbg!(format!("{} {}", v0[i],v1[i]));
                                valid_update = false;
                            }
                        }
                    }
                }
            }
        }

        if valid_update
        {
            sum += update_line[update_line.len() / 2];
            // dbg!(sum);
        }
    }

    sum
}

pub fn main()
{
    let input_file = "input/day_05/input.txt";
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
        let input_file = "input/day_05/test_01.txt";
        let input = fs::read_to_string(input_file).unwrap();

        let result = part_1(&input);

        assert_eq!(result, 143);
    }
}