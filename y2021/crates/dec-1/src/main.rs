use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let times = gt("./resources/input-dec-1");
    println!("{:?}", times);

    let times = gt_walking_average("./resources/input-dec-1");
    println!("{:?}", times);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn gt(input: &str) -> Option<i32> {
    if let Ok(lines) = read_lines(input) {
        let mut previous: Option<i32> = None;
        let mut count = 0;

        for line in lines {
            if let Ok(value) = line {
                if let Ok(i) = value.parse::<i32>() {
                    if let Some(previous) = previous {
                        if i.gt(&previous) {
                            count += 1;
                        }
                    }
                    previous = Some(i);
                }
            }
        }
        return Some(count);
    }
    None
}

fn gt_walking_average(input: &str) -> Option<i32> {
    if let Ok(lines) = read_lines(input) {
        let mut current: (Option<i32>, Option<i32>, Option<i32>) = (None, None, None);
        let mut previous: (Option<i32>, Option<i32>, Option<i32>) = (None, None, None);
        let mut count = 0;

        for line in lines {
            if let Ok(value) = line {
                if let Ok(i) = value.parse::<i32>() {
                    current = (Some(i), previous.0, previous.1);
                    if let (Some(x), Some(y), Some(z)) = previous {
                        let previous = x + y + z;
                        let next = current.0.unwrap() + current.1.unwrap() + current.2.unwrap();
                        // println!("p: {} {} {} c:{} {} {} - 1", z, y, x, current.0.unwrap(), current.1.unwrap(), current.2.unwrap());

                        if next.gt(&previous) {
                            count += 1;
                        }
                    }
                }
                previous = current;
            }
        }
        return Some(count);
    }
    None
}
