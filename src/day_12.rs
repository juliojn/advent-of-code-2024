use std::{collections::VecDeque, fs};
 
fn read_input(input: &str) -> Vec<Vec<char>>
{
    let mut matrix = Vec::new();
    for line in input.lines()
    {
        let line = line.chars().collect();
        matrix.push(line);
    }
    matrix
}
	
fn part_1(input: &str) -> usize
{
    let mut sum = 0;
    let map = read_input(input);
    let length = map.len();
    let width= map[0].len();
    let mut pos_chosen = vec![vec![false;width];length];
    let mut regions = Vec::new();

    for i in 0..length
    {
        for j in 0..width
        {
            if !pos_chosen[i][j]
            {
                let mut region = Vec::new();
                let mut possible_pos= VecDeque::new();
                let letter = map[i][j];
                possible_pos.push_back((i as i64,j as i64));

                while let Some((pos_i,pos_j)) = possible_pos.pop_front()
                {
                    if let Some(file) = map.get(pos_i as usize)
                    {
                        if let Some(c) = file.get(pos_j as usize)
                        {
                            if c == &letter && !pos_chosen[pos_i as usize][pos_j as usize]
                            {
                                region.push((pos_i,pos_j));
                                pos_chosen[pos_i as usize][pos_j as usize] = true;

                                possible_pos.push_back((pos_i-1,pos_j));
                                possible_pos.push_back((pos_i+1,pos_j));
                                possible_pos.push_back((pos_i,pos_j-1));
                                possible_pos.push_back((pos_i,pos_j+1));
                            }
                        }
                    }
                }

                regions.push(region);
            }
        }
    }

    for region in regions
    {
        let area = region.len();
        let mut perimeter = 0;

        for &(i, j) in &region
        {
            let mut lateral_pos = Vec::new();
            lateral_pos.push((i-1,j));
            lateral_pos.push((i+1,j));
            lateral_pos.push((i,j-1));
            lateral_pos.push((i,j+1));
            
            for (pos_i, pos_j) in lateral_pos
            {
                let elem = (pos_i, pos_j);
                if !region.contains(&elem)
                {
                    perimeter += 1;
                }
            }
        }

        sum += area * perimeter;
    }

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
    let input_file = "input/day_12/input.txt";
    // let input_file = "input/day_12/test_01.txt";
    let input = fs::read_to_string(input_file);
    let part = "1";

    match input
    {
        Ok(input) => 
        {
            if part == "1"
            {
                let result = part_1(&input);
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
        let input_file = "input/day_12/test_01.txt";
        let input = fs::read_to_string(input_file).unwrap();

        let result = part_1(&input);

        assert_eq!(result, 140);
    }
    
    #[test]
    fn test_02() {
        let input_file = "input/day_12/test_02.txt";
        let input = fs::read_to_string(input_file).unwrap();

        let result = part_1(&input);

        assert_eq!(result, 772);
    }

    #[test]
    fn test_03() {
        let input_file = "input/day_12/test_03.txt";
        let input = fs::read_to_string(input_file).unwrap();

        let result = part_1(&input);

        assert_eq!(result, 1930);
    }
}
