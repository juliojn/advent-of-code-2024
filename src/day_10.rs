use std::{collections::VecDeque, fs};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Position
{
    row: i64,
    colm: i64,
}

#[derive(Clone, Copy, Debug)]
struct Movement
{
    pos: Position,
    altitude: u64,
}

fn in_bound(pos: Position, heigth: usize, width: usize) -> bool
{
    0 <= pos.row && pos.row < heigth as i64 &&
    0 <= pos.colm && pos.colm < width as i64
}

fn get_value(map: &Vec<Vec<u64>>, pos: Position) -> Option<u64>
{
    let heigth = map.len();
    let width= map[0].len();
    if in_bound(pos, heigth, width)
    {
        return Some(map[pos.row as usize][pos.colm as usize]);
    }
    else
    {
        return None;    
    }
}

fn read_file(input: &str) -> Vec<Vec<u64>>
{
    let mut matrix = Vec::new();
    for line in input.lines()
    {
        let line = line.chars().map(|c: char| c.to_digit(10).unwrap() as u64).collect();
        matrix.push(line);
    }

    matrix
}

fn part_1(input: &str) -> u64
{
    let mut sum = 0;

    let map = read_file(input);
    let mut vector_start = Vec::new();
    let mut vector_end = Vec::new();
    let heigth = map.len();
    let width= map[0].len();

    for i in 0..map.len()
    {
        for j in 0..map[i].len()
        {
            let pos = Position{row:i as i64 ,colm:j as i64};
            match get_value(&map, pos)
            {
                Some(n) =>
                {
                    match n
                    {
                        0 => {vector_start.push(pos);}
                        9 => {vector_end.push(pos);}
                        _ => {}
                    }
                }
                None => {}
            }
        }
    }

    for start in &vector_start
    {
        // let mut routes = 0;
        for end in &vector_end
        {
            let mut movements = VecDeque::new();


            movements.push_back(Movement{pos:start.clone(), altitude:0});

            while let Some(mov) = movements.pop_front()
            {
                match mov.altitude
                {
                    9 =>
                    {
                        if mov.pos == *end
                        {
                            sum += 1;
                            dbg!(&start, &end);
                            break;
                        }
                    }
                    0..=8 =>
                    {
                        let mut new_mov = vec![mov;4];
                        new_mov[0].pos.row -= 1;
                        new_mov[1].pos.row += 1;
                        new_mov[2].pos.colm -= 1;
                        new_mov[3].pos.colm += 1;

                        new_mov[0].altitude += 1;
                        new_mov[1].altitude += 1;
                        new_mov[2].altitude += 1;
                        new_mov[3].altitude += 1;

                        for it in new_mov
                        {
                            if in_bound(it.pos, heigth, width)
                            {
                                if get_value(&map, it.pos).unwrap() == it.altitude
                                {
                                    movements.push_back(it);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }

            
        }
    }


    dbg!(&vector_start);
    dbg!(&vector_end);


    sum
}

fn part_2(input: &str) -> u64
{
    let mut sum = 0;

    let map = read_file(input);
    let mut vector_start = Vec::new();
    let mut vector_end = Vec::new();
    let heigth = map.len();
    let width= map[0].len();

    for i in 0..map.len()
    {
        for j in 0..map[i].len()
        {
            let pos = Position{row:i as i64 ,colm:j as i64};
            match get_value(&map, pos)
            {
                Some(n) =>
                {
                    match n
                    {
                        0 => {vector_start.push(pos);}
                        9 => {vector_end.push(pos);}
                        _ => {}
                    }
                }
                None => {}
            }
        }
    }

    for start in &vector_start
    {
        // let mut routes = 0;
        for end in &vector_end
        {
            let mut movements = VecDeque::new();


            movements.push_back(Movement{pos:start.clone(), altitude:0});

            while let Some(mov) = movements.pop_front()
            {
                match mov.altitude
                {
                    9 =>
                    {
                        if mov.pos == *end
                        {
                            sum += 1;
                            dbg!(&start, &end);
                        }
                    }
                    0..=8 =>
                    {
                        let mut new_mov = vec![mov;4];
                        new_mov[0].pos.row -= 1;
                        new_mov[1].pos.row += 1;
                        new_mov[2].pos.colm -= 1;
                        new_mov[3].pos.colm += 1;

                        new_mov[0].altitude += 1;
                        new_mov[1].altitude += 1;
                        new_mov[2].altitude += 1;
                        new_mov[3].altitude += 1;

                        for it in new_mov
                        {
                            if in_bound(it.pos, heigth, width)
                            {
                                if get_value(&map, it.pos).unwrap() == it.altitude
                                {
                                    movements.push_back(it);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }

            
        }
    }


    dbg!(&vector_start);
    dbg!(&vector_end);


    sum
}

pub fn main()
{
    let input_file = "input/day_10/input.txt";
    // let input_file = "input/day_10/test_01.txt";
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
        let input_file = "input/day_10/test_01.txt";
        let input = fs::read_to_string(input_file).unwrap();

        let result = part_1(&input);

        assert_eq!(result, 36);
    }
    
    #[test]
    fn test_02() {
        let input_file = "input/day_10/test_01.txt";
        let input = fs::read_to_string(input_file).unwrap();

        let result = part_2(&input);

        assert_eq!(result, 81);
    }
}