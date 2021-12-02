use itertools::Itertools;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    if let Ok(numbers) = read_input("./resources/input-dec-1") {
        let times = cmp_pref_next(numbers.clone());
        println!("{:?}", times);

        let times = cmp_abc_bcd(numbers.clone());
        println!("{:?}", times);
    }
}

fn cmp_pref_next(numbers: Vec<i32>) -> i32 {
    let mut count = 0;
    for (pref, current) in numbers.into_iter().tuple_windows() {
        if current.gt(&pref) {
            count += 1;
        }
    }
    count
}

fn cmp_abc_bcd(numbers: Vec<i32>) -> i32 {
    let mut count = 0;
    for (a, b, c, d) in numbers.into_iter().tuple_windows() {
        let pref = a + b + c;
        let current = b + c + d;
        if pref < current {
            count += 1;
        }
    }
    count
}

fn read_input<P>(filename: P) -> io::Result<Vec<i32>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let x: Vec<i32> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();
    Ok(x)
}