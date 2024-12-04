use std::{fs};

#[derive(Debug, Clone, Copy)]
enum Displacement
{
    NONE,
    FORWARD,
    BACKWARD
}

fn str_to_matrix(input: &str) -> Vec<Vec<char>>
{
    // let mut matrix: Vec< Vec<char> > = Vec::new();
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in input.lines()
    {
        let mut matrix_line: Vec<char> = Vec::new();
        let file_len = line.len();

        for j in 0..file_len
        {
            matrix_line.push(line.as_bytes()[j] as char);
        }

        matrix.push(matrix_line);
    }

    matrix
}

fn search(matrix: &Vec<Vec<char>>, file_search: Displacement, colm_search: Displacement) -> u64
{
    let mut sum = 0;
    let files = matrix.len() as isize;
    let columns = matrix[0].len() as isize;

    dbg!(file_search, colm_search);
    for i in 0..files as usize
    {
        for j in 0..columns as usize
        {
            if matrix[i][j] == 'X'
            {
                dbg!(i, j, &matrix[i][j]);
                let next_i = match file_search
                {
                    Displacement::NONE => {vec![i as isize; 3]},
                    Displacement::FORWARD => {vec![(i as isize)+1,(i as isize)+2 ,(i as isize)+3]},
                    Displacement::BACKWARD=> {vec![(i as isize)-1,(i as isize)-2 ,(i as isize)-3]},
                };
                dbg!(&next_i);

                let next_j = match colm_search
                {
                    Displacement::NONE => {vec![j as isize; 3]},
                    Displacement::FORWARD => {vec![(j as isize)+1,(j as isize)+2 ,(j as isize)+3]},
                    Displacement::BACKWARD=> {vec![(j as isize)-1,(j as isize)-2 ,(j as isize)-3]},
                };
                dbg!(&next_j);

                let in_range: bool =    0 <= next_i[0] && next_i[0] < files &&
                                        0 <= next_i[1] && next_i[1] < files &&
                                        0 <= next_i[2] && next_i[2] < files &&
                                        0 <= next_j[0] && next_j[0] < columns &&
                                        0 <= next_j[1] && next_j[1] < columns &&
                                        0 <= next_j[2] && next_j[2] < columns;

                if in_range && matrix[next_i[0] as usize][next_j[0] as usize] == 'M' && 
                    matrix[next_i[1] as usize][next_j[1] as usize] == 'A' &&
                    matrix[next_i[2] as usize][next_j[2] as usize] == 'S'
                {
                    sum = sum + 1;
                    dbg!(sum);
                }
            }
        }
    }

    dbg!(sum);
    sum
}


fn search_mas(matrix: &Vec<Vec<char>>, i :isize, j: isize, file_search: Displacement, colm_search: Displacement) -> bool
{
    let files = matrix.len() as isize;
    let columns = matrix[0].len() as isize;

    dbg!(file_search, colm_search);
    let next_i = match file_search
    {
        Displacement::NONE => {vec![i; 3]},
        Displacement::FORWARD => {vec![(i as isize)+0,(i as isize)+1 ,(i as isize)+2]},
        Displacement::BACKWARD=> {vec![(i as isize)-0,(i as isize)-1 ,(i as isize)-2]},
    };
    dbg!(&next_i);

    let next_j = match colm_search
    {
        Displacement::NONE => {vec![j; 3]},
        Displacement::FORWARD => {vec![(j as isize)+0,(j as isize)+1 ,(j as isize)+2]},
        Displacement::BACKWARD=> {vec![(j as isize)-0,(j as isize)-1 ,(j as isize)-2]},
    };

    let in_range: bool =    0 <= next_i[0] && next_i[0] < files &&
                            0 <= next_i[1] && next_i[1] < files &&
                            0 <= next_i[2] && next_i[2] < files &&
                            0 <= next_j[0] && next_j[0] < columns &&
                            0 <= next_j[1] && next_j[1] < columns &&
                            0 <= next_j[2] && next_j[2] < columns;

    if in_range && matrix[next_i[0] as usize][next_j[0] as usize] == 'M' && 
        matrix[next_i[1] as usize][next_j[1] as usize] == 'A' &&
        matrix[next_i[2] as usize][next_j[2] as usize] == 'S'

    {
        return true;
    }

    return false;
}

fn part_1(input: &str) -> u64
{
    let mut sum: u64 = 0;

    let matrix = str_to_matrix(input);
    sum += search(&matrix, Displacement::NONE, Displacement::FORWARD);
    sum += search(&matrix, Displacement::NONE, Displacement::BACKWARD);
    sum += search(&matrix, Displacement::FORWARD, Displacement::NONE);
    sum += search(&matrix, Displacement::FORWARD, Displacement::FORWARD);
    sum += search(&matrix, Displacement::FORWARD, Displacement::BACKWARD);
    sum += search(&matrix, Displacement::BACKWARD, Displacement::NONE);
    sum += search(&matrix, Displacement::BACKWARD, Displacement::FORWARD);
    sum += search(&matrix, Displacement::BACKWARD, Displacement::BACKWARD);

    sum
}

#[allow(unused_variables)]
fn part_2(input: &str) -> u64
{
    let mut sum: u64 = 0;
    let matrix = str_to_matrix(input);
    let files = matrix.len();
    let columns = matrix[0].len();

    for i in 0..files as isize
    {
        for j in 0..columns as isize
        {
            if matrix[i as usize][j as usize] == 'A'
            {
                if (search_mas(&matrix, i-1, j-1, Displacement::FORWARD, Displacement::FORWARD) ||
                    search_mas(&matrix, i+1, j+1, Displacement::BACKWARD, Displacement::BACKWARD)) &&
                   (search_mas(&matrix, i-1, j+1, Displacement::FORWARD, Displacement::BACKWARD) ||
                    search_mas(&matrix, i+1, j-1, Displacement::BACKWARD, Displacement::FORWARD))
                {
                    sum += 1;
                }
            }
        }
    }

    sum
    
}

pub fn main()
{
    let input_file = "input/day_04/input.txt";
    let input = fs::read_to_string(input_file);
    // let part = env::args().nth(1).unwrap();
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
        let input_file = "input/day_04/test_01.txt";
        let input = fs::read_to_string(input_file).unwrap();

        let result = part_1(&input);

        assert_eq!(result, 4);
    }
    
    #[test]
    fn test_02() {
        let input_file = "input/day_04/test_02.txt";
        let input = fs::read_to_string(input_file).unwrap();

        let result = part_1(&input);

        assert_eq!(result, 18);
    }    

    #[test]
    fn test_03() {
        let input_file = "input/day_04/test_03.txt";
        let input = fs::read_to_string(input_file).unwrap();

        let result = part_2(&input);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_04() {
        let input_file = "input/day_04/test_04.txt";
        let input = fs::read_to_string(input_file).unwrap();

        let result = part_2(&input);

        assert_eq!(result, 9);
    }
}