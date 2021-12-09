use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    if let Ok(numbers) = read_input("./resources/input-dec-7") {
        let (best_position, min_fuel) = calulation_1(numbers);
        println!("best is: {} fuel: {}", best_position, min_fuel)
    }

    if let Ok(numbers) = read_input("./resources/input-dec-7") {
        let (best_position, min_fuel) = calulation_2(numbers);
        println!("best is: {} fuel: {}", best_position, min_fuel)
    }
}

fn calulation_1(numbers: Vec<i32>) -> (i32, i64) {
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();

    let mut min_fuel: i64 = numbers.iter().sum::<i32>() as i64;
    let mut best_position: i32 = 0;
    for i in min..max {
        let mut current_fuel = 0;
        for x in numbers.iter() {
            current_fuel += (i - x).abs() as i64;
        }
        if current_fuel < min_fuel {
            min_fuel = current_fuel;
            best_position = i;
        }
    }
    (best_position, min_fuel)
}

fn calulation_2(numbers: Vec<i32>) -> (i32, i64) {
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();

    let mut min_fuel: i64 = numbers.iter().map(|x| (1..=*x).sum::<i32>()).sum::<i32>() as i64;
    let mut best_position: i32 = 0;
    for i in min..max {
        let mut current_fuel = 0;
        for x in numbers.iter() {
            let steps = (i - x).abs();
            current_fuel += (1..=steps).sum::<i32>() as i64;
        }
        if current_fuel < min_fuel {
            min_fuel = current_fuel;
            best_position = i;
        }
    }
    (best_position, min_fuel)
}

fn read_input<P>(filename: P) -> io::Result<Vec<i32>>
where
    P: AsRef<Path>,
{
    let mut numbers: Vec<i32> = Vec::new();

    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut numbers_input: String = String::new();
    if reader.read_line(&mut numbers_input).is_ok() {
        numbers = numbers_input
            .trim()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
    }

    Ok(numbers)
}

#[cfg(test)]
mod test {
    use crate::{calulation_1, calulation_2};

    #[test]
    fn solution_1() {
        let numbers = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(calulation_1(numbers).1, 37i64)
    }
    #[test]
    fn solution_2() {
        let numbers = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(calulation_2(numbers).1, 168i64)
    }
}
