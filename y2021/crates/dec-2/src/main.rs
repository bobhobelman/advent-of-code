use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Forward(i32),
}

#[derive(Debug)]
struct Location {
    height: i32,
    distance: i32,
}

impl Direction {
    fn create(direction: &str, amount: i32) -> Direction {
        match direction {
            "up" => Direction::Up(amount),
            "down" => Direction::Down(amount),
            "forward" => Direction::Forward(amount),
            _ => panic!("Could not match: {}", direction),
        }
    }

    fn process(&self, location: &mut Location) -> Location{
        match self {
            Direction::Up(i) => Location {
                height: location.height - i,
                distance: location.distance,
            },
            Direction::Down(i) => Location {
                height: location.height + i,
                distance: location.distance,
            },
            Direction::Forward(i) => Location {
                height: location.height,
                distance: location.distance + i,
            },
        }
    }
}

impl Location {
    fn new() -> Location {
        Location {
            height: 0,
            distance: 0,
        }
    }
}

fn main() {
    let mut location = Location::new();
    if let Ok(commands) = read_input("./resources/input-dec-2") {
        for command in commands {
            location = command.process(&mut location);
        }
    }
    println!("{:?}", location);
    println!("{}", location.height * location.distance)
}

fn read_input<P>(filename: P) -> io::Result<Vec<Direction>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut x: Vec<Direction> = Vec::new();
    io::BufReader::new(file).lines().for_each(|l| {
        let string = l.unwrap();
        let line = string.split_once(" ");
        if let Some((direction, amount)) = line {
            x.push(Direction::create(direction, amount.parse::<i32>().unwrap()));
        }
    });
    Ok(x)
}
