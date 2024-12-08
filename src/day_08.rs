use std::{fs, str};

fn in_bound(i: i64, j: i64, length: i64, width: i64) -> bool
{
    return i >= 0 && j >= 0 && i < length && j < width;
}

fn str_to_matrix(input: &str) -> Vec<Vec<char>>
{
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in input.lines()
    {
        let mut matrix_line: Vec<char> = Vec::new();
        let file_len: usize = line.len();

        for j in 0..file_len
        {
            matrix_line.push(line.as_bytes()[j] as char);
        }

        matrix.push(matrix_line);
    }

    matrix
}

fn part_1(input: &str) -> u64
{
    let mut sum = 0;
    let map = str_to_matrix(input);
    let length = map.len() as i64;
    let width = map[0].len() as i64;
    let mut antinode = vec![vec!['.';width as usize];length as usize];

    for node_i in 0..length as i64
    {
        for node_j in 0..width as i64
        {
            if antinode[node_i as usize][node_j as usize] != '#'
            {
                for map_i in 0..length
                {
                    for map_j in 0..width
                    {
                        let c = map[map_i as usize][map_j as usize];
                        if c != '.' && map_i != node_i && map_j != node_j
                        {
                            let new_i = node_i + 2*(map_i - node_i);
                            let new_j = node_j + 2*(map_j - node_j);

                            if in_bound(new_i, new_j, length, width)
                            {
                                if map[new_i as usize][new_j as usize] == c
                                {
                                    antinode[node_i as usize][node_j as usize] = '#';
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    for i in 0..length as usize
    {
        // println!("{:?}", antinode[i]);
        for j in 0..width as usize
        {
            if antinode[i][j] == '#'
            {
                sum += 1;
            }
        }
    }


    sum
}

pub fn main()
{
    let input_file = "input/day_08/input.txt";
    // let input_file = "input/day_08/test_01.txt";
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
        let input_file = "input/day_08/test_01.txt";
        let input = fs::read_to_string(input_file).unwrap();

        let result = part_1(&input);

        assert_eq!(result, 14);
    }
}