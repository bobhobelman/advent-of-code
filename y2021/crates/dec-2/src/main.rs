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

impl Direction {
    fn create(direction: &str, amount: i32) -> Direction {
        match direction {
            "up" => Direction::Up(amount),
            "down" => Direction::Down(amount),
            "forward" => Direction::Forward(amount),
            _ => panic!("Could not match: {}", direction),
        }
    }
}

#[derive(Debug)]
struct Location {
    height: i32,
    distance: i32,
}

impl Location {
    fn new() -> Location {
        Location {
            height: 0,
            distance: 0,
        }
    }
}

#[derive(Debug)]
struct Submarine {
    location: Location,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            location: Location::new(),
        }
    }

    fn navigate(&mut self, direction: & Direction) {
        match direction {
            Direction::Up(i) => {
                self.location.height -= i;
            },
            Direction::Down(i) => {
                self.location.height += i;
            },
            Direction::Forward(i) => {
                self.location.distance += i;
            },
        }
    }
}

fn main() {
    let mut sub = Submarine::new();
    if let Ok(commands) = read_input("./resources/input-dec-2") {
        for command in commands {
            sub.navigate(&command);
        }
    }
    println!("{:?}", sub);
    println!("{}", sub.location.height * sub.location.distance)
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
