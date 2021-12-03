use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let input: Vec<Vec<u8>> = read_input("./resources/input-dec-3").unwrap();
    println!("power: {}", calculate_power(&input));
    println!("support: {}", calculate_life_support(&input));
}

fn bin_vec_to_int(bin: &[u8]) -> i32 {
    i32::from_str_radix(&String::from_utf8(bin.to_owned()).unwrap(), 2).unwrap()
}

fn calculate_power(input: &[Vec<u8>]) -> i32 {
    let mut gamma: Vec<u8> = vec![];
    let mut epsilon: Vec<u8> = vec![];
    for i in 0..input[0].len() {
        let ones = input.iter().filter(|x| x[i] == b'1').count();
        let zeros = input.iter().filter(|x| x[i] == b'0').count();
        assert_eq!(input.len(), ones + zeros);

        if ones > zeros {
            gamma.push(b'1');
            epsilon.push(b'0');
        } else {
            gamma.push(b'0');
            epsilon.push(b'1');
        }
    }
    let gamma = bin_vec_to_int(&gamma);
    let epsilon = bin_vec_to_int(&epsilon);
    gamma * epsilon
}

fn calculate_life_support(input: &[Vec<u8>]) -> i32 {
    let mut oxygen = input.to_owned();
    for c in 0..oxygen[0].len() {
        let mut ones = 0;
        let mut zeros = 0;
        for line in &oxygen {
            if line[c] == b'1' {
                ones += 1;
            } else {
                zeros += 1;
            }
        }
        if ones >= zeros {
            oxygen.retain(|x| x[c] == b'1');
        } else {
            oxygen.retain(|x| x[c] == b'0');
        }
        if oxygen.len() == 1 {
            break;
        }
    }

    let mut co2 = input.to_owned();
    for c in 0..co2[0].len() {
        let mut ones = 0;
        let mut zeros = 0;
        for line in &co2 {
            if line[c] == b'1' {
                ones += 1;
            } else {
                zeros += 1;
            }
        }
        if zeros <= ones {
            co2.retain(|x| x[c] == b'0');
        } else {
            co2.retain(|x| x[c] == b'1');
        }
        if co2.len() == 1 {
            break;
        }
    }
    let oxygen = bin_vec_to_int(&oxygen[0]);
    let co2 = bin_vec_to_int(&co2[0]);
    oxygen * co2
}

fn read_input<P>(filename: P) -> io::Result<Vec<Vec<u8>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut x: Vec<Vec<u8>> = Vec::new();
    io::BufReader::new(file).lines().for_each(|line| {
        if let Ok(line) = line {
            x.push(line.into_bytes());
        }
    });
    Ok(x)
}

#[cfg(test)]
mod tests {
    use crate::{calculate_life_support, calculate_power};

    #[test]
    fn test() {
        let input: Vec<Vec<u8>> = vec![
            vec![b'0', b'0', b'1', b'0', b'0'],
            vec![b'1', b'1', b'1', b'1', b'0'],
            vec![b'1', b'0', b'1', b'1', b'0'],
            vec![b'1', b'0', b'1', b'1', b'1'],
            vec![b'1', b'0', b'1', b'0', b'1'],
            vec![b'0', b'1', b'1', b'1', b'1'],
            vec![b'0', b'0', b'1', b'1', b'1'],
            vec![b'1', b'1', b'1', b'0', b'0'],
            vec![b'1', b'0', b'0', b'0', b'0'],
            vec![b'1', b'1', b'0', b'0', b'1'],
            vec![b'0', b'0', b'0', b'1', b'0'],
            vec![b'0', b'1', b'0', b'1', b'0'],
        ];
        assert_eq!(calculate_power(&input), 198);
        assert_eq!(calculate_life_support(&input), 230);
    }
}
