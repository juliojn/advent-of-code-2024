use core::panic;
use std::{fs};

#[derive(Clone, Copy, Debug)]
enum Orientation
{
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

fn calculcate_next_pos(pos: (isize, isize), orientation: Orientation) -> (isize, isize)
{
    match orientation
    {
        Orientation::UP =>      {(pos.0-1,pos.1)},
        Orientation::DOWN =>    {(pos.0+1,pos.1)},
        Orientation::LEFT =>    {(pos.0,pos.1-1)},
        Orientation::RIGHT =>   {(pos.0,pos.1+1)}
    }
}

#[allow(unused_variables)]
fn position_valid(pos: (isize, isize), orientation: Orientation, map: &Vec<Vec<char>>) -> bool
{
    let files = map.len();
    let columns = map[0].len();

    0 <= pos.0 && pos.0 < files as isize &&
    0 <= pos.1 && pos.1 < columns as isize

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
    let files = map.len();
    let columns = map[0].len();
    let mut map_visited = vec![vec!['.';columns];files];
    let mut pos = (-1,-1);
    let mut orientation = Orientation::UP;
    
    for i in 0..map.len() as isize
    {
        for j in 0..map[i as usize].len() as isize
        {
            match map[i as usize][j as usize]
            {
                '^' => {
                    pos = (i,j);
                    map_visited[i as usize][j as usize] = 'X';
                    orientation = Orientation::UP;
                },
                'v' => {
                    pos = (i,j);
                    map_visited[i as usize][j as usize] = 'X';
                    orientation = Orientation::DOWN;
                },
                '<' => {
                    pos = (i,j);
                    map_visited[i as usize][j as usize] = 'X';
                    orientation = Orientation::LEFT;
                },
                '>' => {
                    pos = (i,j);
                    map_visited[i as usize][j as usize] = 'X';
                    orientation = Orientation::RIGHT;
                },
                _ => {},
            }
        }
    }

    dbg!(pos, &orientation);

    while position_valid(pos, orientation, &map) {
        let mut next_pos;
        let mut next_orientation = orientation;

        map_visited[pos.0 as usize][pos.1 as usize] = 'X';
    
        next_pos = calculcate_next_pos(pos, orientation);

        while position_valid(next_pos, next_orientation, &map) &&
            map[next_pos.0 as usize][next_pos.1 as usize] == '#'
        {
            next_orientation = match orientation {
                Orientation::UP => Orientation::RIGHT,
                Orientation::RIGHT => Orientation::DOWN,
                Orientation::DOWN => Orientation::LEFT,
                Orientation::LEFT => Orientation::UP,
            };
        
            next_pos = calculcate_next_pos(pos, next_orientation);
        }
        
        pos = next_pos;
        orientation = next_orientation;
        dbg!(&next_orientation,pos);
    }

    for i in 0..map.len() as isize
    {
        for j in 0..map[i as usize].len() as isize
        {
            if map_visited[i as usize][j as usize] == 'X'
            {
                print!("X");
                sum += 1;
            }
            else
            {
                print!(".");
            }
        }
        println!("");
    }

    
    sum
}

fn part_2(input: &str) -> u64
{
    let mut sum = 0;

    let mut map = str_to_matrix(input);
    let files = map.len();
    let columns = map[0].len();
    let mut initial_pos = (-1,-1);
    let mut initial_orientation = Orientation::UP;
    
    for i in 0..map.len() as isize
    {
        for j in 0..map[i as usize].len() as isize
        {
            match map[i as usize][j as usize]
            {
                '^' => {
                    initial_pos = (i,j);
                    initial_orientation = Orientation::UP;
                },
                '>' => {
                    initial_pos = (i,j);
                    initial_orientation = Orientation::RIGHT;
                },
                'v' => {
                    initial_pos = (i,j);
                    initial_orientation = Orientation::DOWN;
                },
                '<' => {
                    initial_pos = (i,j);
                    initial_orientation = Orientation::LEFT;
                },

                _ => {},
            }
        }
    }


    for i in 0..map.len() as isize
    {
        for j in 0..map[i as usize].len() as isize
        {
            if map[i as usize][j as usize] == '.'
            {
                let mut map_visited= vec![vec![vec!{false;4};columns];files];
                map[i as usize][j as usize] = '#';
                let mut pos = initial_pos;
                let mut orientation = initial_orientation;
                while position_valid(pos, orientation, &map) && 
                        map_visited[pos.0 as usize][pos.1 as usize][orientation as usize] == false
                {
                    let mut next_pos;
                    let mut next_orientation = orientation;

                    match orientation {
                        Orientation::UP     => map_visited[pos.0 as usize][pos.1 as usize][0] = true,
                        Orientation::RIGHT  => map_visited[pos.0 as usize][pos.1 as usize][1] = true,
                        Orientation::DOWN   => map_visited[pos.0 as usize][pos.1 as usize][2] = true,
                        Orientation::LEFT   => map_visited[pos.0 as usize][pos.1 as usize][3] = true,
                    }
                
                    next_pos = calculcate_next_pos(pos, orientation);

                    while position_valid(next_pos, next_orientation, &map) &&
                        map[next_pos.0 as usize][next_pos.1 as usize] == '#'
                    {
                        next_orientation = match next_orientation {
                            Orientation::UP => Orientation::RIGHT,
                            Orientation::RIGHT => Orientation::DOWN,
                            Orientation::DOWN => Orientation::LEFT,
                            Orientation::LEFT => Orientation::UP,
                        };
                    
                        next_pos = calculcate_next_pos(pos, next_orientation);
                    }
                    
                    pos = next_pos;
                    orientation = next_orientation;
                }
                if position_valid(pos, orientation, &map)
                {
                    sum += 1
                }
                map[i as usize][j as usize] = '.';
            }
        }
    }

    sum
}

pub fn main()
{
    let input_file = "input/day_06/input.txt";
    // let input_file = "input/day_06/test_01.txt";
    let input = fs::read_to_string(input_file);
    let part = "2";

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
        let input_file = "input/day_06/test_01.txt";
        let input = fs::read_to_string(input_file).unwrap();

        let result = part_1(&input);

        assert_eq!(result, 41);
    }

    #[test]
    fn test_02() {
        let input_file = "input/day_06/test_01.txt";
        let input = fs::read_to_string(input_file).unwrap();

        let result = part_2(&input);

        assert_eq!(result, 6);
    }
}