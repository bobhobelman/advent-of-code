use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let input: Vec<Vec<char>> = read_input("./resources/input-dec-3").unwrap();
    // "1101".chars().for_each(|c| println!("{:?}", c.to_digit(2).unwrap()));
    println!("power: {}", calculate_power(input.clone()));
    println!("support: {}", calculate_life_support(input.clone()));
}

fn calculate_power(input: Vec<Vec<char>>) -> isize {
    let mut gamma: Vec<char> = vec![];
    let mut epsilon: Vec<char> = vec![];
    for i in 0..input[0].len() {
        let ones = input
            .iter()
            .filter(|x| x[i].eq_ignore_ascii_case(&'1'))
            .count();
        let zeros = input
            .iter()
            .filter(|x| x[i].eq_ignore_ascii_case(&'0'))
            .count();
        assert_eq!(input.len(), ones + zeros);

        if ones > zeros {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    let gamma = isize::from_str_radix(String::from_iter(gamma).as_str(), 2).unwrap();
    let epsilon = isize::from_str_radix(String::from_iter(epsilon).as_str(), 2).unwrap();
    gamma * epsilon
}

fn calculate_life_support(input: Vec<Vec<char>>) -> isize {
    let mut oxygen = input.clone();
    for c in 0..oxygen[0].len() {
        let mut ones = 0;
        let mut zeros = 0;
        for line in &oxygen {
            if line[c].eq_ignore_ascii_case(&'1') {
                ones += 1;
            } else {
                zeros += 1;
            }
        }
        if ones >= zeros {
            oxygen.retain(|x| x[c].eq_ignore_ascii_case(&'1'));
        } else {
            oxygen.retain(|x| x[c].eq_ignore_ascii_case(&'0'));
        }
        if oxygen.len() == 1 {
            break
        }
    }


    let mut co2 = input.clone();
    for c in 0..co2[0].len() {
        let mut ones = 0;
        let mut zeros = 0;
        for line in &co2 {
            if line[c].eq_ignore_ascii_case(&'1') {
                ones += 1;
            } else {
                zeros += 1;
            }
        }
        if zeros <= ones {
            co2.retain(|x| x[c].eq_ignore_ascii_case(&'0'));
        } else {
            co2.retain(|x| x[c].eq_ignore_ascii_case(&'1'));
        }
        if co2.len() == 1 {
            break
        }
    }
    let oxygen = isize::from_str_radix(String::from_iter(&oxygen[0]).as_str(), 2).unwrap();
    let co2 = isize::from_str_radix(String::from_iter(&co2[0]).as_str(), 2).unwrap();
    oxygen * co2
}

fn read_input<P>(filename: P) -> io::Result<Vec<Vec<char>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut x: Vec<Vec<char>> = Vec::new();
    io::BufReader::new(file).lines().for_each(|line| {
        let bl = Vec::from_iter(line.unwrap().chars());
        x.push(bl);
    });
    Ok(x)
}

#[cfg(test)]
mod tests {
    use crate::{calculate_life_support, calculate_power};

    #[test]
    fn test_part_1() {
        let input: Vec<Vec<char>> = vec![
            vec!['0', '0', '1', '0', '0'],
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '0', '1', '1', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '0', '1', '0', '1'],
            vec!['0', '1', '1', '1', '1'],
            vec!['0', '0', '1', '1', '1'],
            vec!['1', '1', '1', '0', '0'],
            vec!['1', '0', '0', '0', '0'],
            vec!['1', '1', '0', '0', '1'],
            vec!['0', '0', '0', '1', '0'],
            vec!['0', '1', '0', '1', '0'],
        ];
        assert_eq!(calculate_power(input), 198)
    }

    #[test]
    fn test_part_2() {
        let input: Vec<Vec<char>> = vec![
            vec!['0', '0', '1', '0', '0'],
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '0', '1', '1', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '0', '1', '0', '1'],
            vec!['0', '1', '1', '1', '1'],
            vec!['0', '0', '1', '1', '1'],
            vec!['1', '1', '1', '0', '0'],
            vec!['1', '0', '0', '0', '0'],
            vec!['1', '1', '0', '0', '1'],
            vec!['0', '0', '0', '1', '0'],
            vec!['0', '1', '0', '1', '0'],
        ];

        assert_eq!(calculate_life_support(input), 230)
    }
}
