use std::fs;

fn part_1(input: &str) -> u64
{
    let mut sum = 0;

    for line in input.lines()
    {
        let disk_map: Vec<_> = line.chars().map(|x| x.to_digit(10).unwrap() as i64).collect();
        let mut block = Vec::new();
        // dbg!(&disk_map);
        let mut index: i64 = 0;

        for i in 0..disk_map.len()
        {
            let number = disk_map[i];
            if i % 2 == 0
            {
                for _j in 0..number as isize
                {
                    block.push(index);
                }

                index += 1;
            }
            else
            {
                for _j in 0..number as isize
                {
                    block.push(-1);
                }
            }
        }

        let mut i  = 0;
        let mut j  = block.len()-1;

        while i < j
        {
            if block[i] == -1 && block[j] != -1
            {
                block.swap(i, j);
                i += 1;
                j -= 1;
            }
            else if block[i] != -1
            {
                i += 1;
            }
            else    // block[j] == -1
            {
                j-= 1;
            }
        }
        
        // dbg!(block.iter().collect::<String>());
        for i in 0..block.len()
        {
            if block[i] != -1
            {
                sum += i as i64 * block[i];
            }
            else
            {
                break;
            }
            // }
        }
    }


    sum as u64
}

pub fn main()
{
    let input_file = "input/day_09/input.txt";
    // let input_file = "input/day_09/test_01.txt";
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
        let input_file = "input/day_09/test_01.txt";
        let input = fs::read_to_string(input_file).unwrap();

        let result = part_1(&input);

        assert_eq!(result, 1928);
    }
}