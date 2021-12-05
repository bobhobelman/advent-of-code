use ansi_term::Style;
use grid::*;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
struct Thickness {
    number: i32,
}

impl Display for Thickness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let number = match self.number {
            0 => Style::new().bold().paint(format!("{:03}", 0)),
            x => Style::new().paint(format!("{:03}", x)),
        };
        write!(f, "{} ", number)
    }
}

impl Default for Thickness {
    fn default() -> Self {
        Self { number: 0 }
    }
}

impl Thickness {
    fn add_cloud(&mut self) {
        self.number += 1;
    }

    fn cloudy(&self) -> bool {
        self.number > 1
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Location {
    x: usize,
    y: usize,
}

impl FromStr for Location {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((x, y)) = s.split_once(',') {
            Ok(Location {
                x: x.parse::<usize>().unwrap(),
                y: y.parse::<usize>().unwrap(),
            })
        } else {
            panic!("Could not parse location.")
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Cloud {
    from: Location,
    to: Location,
}

impl FromStr for Cloud {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((first, second)) = s.split_once(" -> ") {
            Ok(Cloud {
                from: first.parse::<Location>().unwrap(),
                to: second.parse::<Location>().unwrap(),
            })
        } else {
            panic!("Could not parse cloud.")
        }
    }
}

fn print(grid: Grid<Thickness>) {
    for row in 0..grid.rows() {
        for number in grid.iter_row(row) {
            print!("{}", number);
        }
        println!();
    }
}

fn main() {
    if let Ok(clouds) = read_input("./resources/input-dec-5") {
        let cloudy_points = process_clouds(clouds);
        println!("Number of cloudy points: {}", cloudy_points);
    }
}

fn process_clouds(clouds: Vec<Cloud>) -> usize {
    let mut grid: Grid<Thickness> = Grid::new(1000, 1000);
    for cloud in clouds {
        match cloud {
            cloud if cloud.from.le(&cloud.to) && cloud.from.x == cloud.to.x => {
                for y in cloud.from.y..=cloud.to.y {
                    if let Some(location) = grid.get_mut(cloud.from.x, y) {
                        location.add_cloud();
                    }
                }
            }
            cloud if cloud.from.le(&cloud.to) && cloud.from.y == cloud.to.y => {
                for x in cloud.from.x..=cloud.to.x {
                    if let Some(location) = grid.get_mut(x, cloud.from.y) {
                        location.add_cloud();
                    }
                }
            }
            cloud if cloud.from.gt(&cloud.to) && cloud.from.x == cloud.to.x => {
                for y in cloud.to.y..=cloud.from.y {
                    if let Some(location) = grid.get_mut(cloud.from.x, y) {
                        location.add_cloud();
                    }
                }
            }
            cloud if cloud.from.gt(&cloud.to) && cloud.from.y == cloud.to.y => {
                for x in cloud.to.x..=cloud.from.x {
                    if let Some(location) = grid.get_mut(x, cloud.from.y) {
                        location.add_cloud();
                    }
                }
            }
            cloud => {
                println!("No needto handle: {:?}", cloud);
            }
        }
    }
    grid.iter().filter(|location| location.cloudy()).count()
}

fn read_input<P>(filename: P) -> io::Result<Vec<Cloud>>
where
    P: AsRef<Path>,
{
    let mut clouds: Vec<Cloud> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    reader.lines().for_each(|line| {
        if let Ok(line) = line {
            clouds.push(line.parse::<Cloud>().unwrap());
        }
    });
    Ok(clouds)
}

#[cfg(test)]
mod tests {
    use crate::{process_clouds, read_input};

    #[test]
    fn solution_1() {
        if let Ok(clouds) = read_input("./resources/test-input-dec-5") {
            let cloudy_points = process_clouds(clouds);
            println!("Number of cloudy points: {}", cloudy_points);
            assert_eq!(cloudy_points, 5)
        }
    }
}
