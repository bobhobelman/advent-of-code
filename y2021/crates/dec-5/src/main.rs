use ansi_term::{Colour, Style};
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
            0 => Style::new().fg(Colour::Blue).paint("#"),
            1 => Style::new().fg(Colour::Green).bold().paint("#"),
            _ => Style::new().fg(Colour::Red).bold().paint("#"),
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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Cloud {
    from: Location,
    to: Location,
}

impl Iterator for Cloud {
    type Item = Cloud;

    fn next(&mut self) -> Option<Self::Item> {
        match &self {
            cloud if cloud.from.x == cloud.to.x && cloud.from.y < cloud.to.y => {
                self.from.y += 1;
            }
            cloud if cloud.from.x == cloud.to.x && cloud.from.y > cloud.to.y => {
                self.from.y -= 1;
            }
            cloud if cloud.from.x < cloud.to.x && cloud.from.y == cloud.to.y => self.from.x += 1,
            cloud if cloud.from.x > cloud.to.x && cloud.from.y == cloud.to.y => self.from.x -= 1,
            cloud if cloud.from.x < cloud.to.x && cloud.from.y < cloud.to.y => {
                self.from.x += 1;
                self.from.y += 1;
            }
            cloud if cloud.from.x > cloud.to.x && cloud.from.y > cloud.to.y => {
                self.from.x -= 1;
                self.from.y -= 1;
            }
            cloud if cloud.from.x < cloud.to.x && cloud.from.y > cloud.to.y => {
                self.from.x += 1;
                self.from.y -= 1;
            }
            cloud if cloud.from.x > cloud.to.x && cloud.from.y < cloud.to.y => {
                self.from.x -= 1;
                self.from.y += 1;
            }
            _ => {
                return None;
            }
        }
        Some(*self)
    }
}

impl FromStr for Cloud {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((from, to)) = s.split_once(" -> ") {
            Ok(Cloud {
                from: from.parse::<Location>().unwrap(),
                to: to.parse::<Location>().unwrap(),
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
        let map = process_clouds_1(clouds);
        let cloudy_points = map.iter().filter(|location| location.cloudy()).count();
        println!("Number of cloudy points: {}", cloudy_points);
    }

    if let Ok(clouds) = read_input("./resources/input-dec-5") {
        let map = process_clouds_2(clouds);
        let cloudy_points = map.iter().filter(|location| location.cloudy()).count();
        println!("Number of cloudy points: {}", cloudy_points);
    }
}

fn process_clouds_1(clouds: Vec<Cloud>) -> Grid<Thickness> {
    let max = match clouds.iter().map(|cloud| cloud.from.max(cloud.to)).max() {
        Some(location) => location.x.max(location.y),
        None => {
            panic!("No max found!")
        }
    };

    let mut grid: Grid<Thickness> = Grid::new(max + 1, max + 1);
    for cloud in clouds {
        if cloud.from.x == cloud.to.x || cloud.from.y == cloud.to.y {
            if let Some(location) = grid.get_mut(cloud.from.y, cloud.from.x) {
                location.add_cloud();
            }
            for cloud in cloud {
                println!("{:?}", cloud);
                if let Some(location) = grid.get_mut(cloud.from.y, cloud.from.x) {
                    location.add_cloud();
                }
            }
        }
    }
    grid
}

fn process_clouds_2(clouds: Vec<Cloud>) -> Grid<Thickness> {
    let max = match clouds.iter().map(|cloud| cloud.from.max(cloud.to)).max() {
        Some(location) => location.x.max(location.y),
        None => {
            panic!("No max found!")
        }
    };

    let mut grid: Grid<Thickness> = Grid::new(max + 1, max + 1);
    for cloud in clouds {
        if let Some(location) = grid.get_mut(cloud.from.y, cloud.from.x) {
            location.add_cloud();
        }
        for cloud in cloud {
            if let Some(location) = grid.get_mut(cloud.from.y, cloud.from.x) {
                location.add_cloud();
            }
        }
    }
    grid
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
    use crate::{print, process_clouds_1, process_clouds_2, read_input, Cloud, Location};

    #[test]
    fn solution_1() {
        match read_input("../../resources/test-input-dec-5")
        {
            Ok(clouds) => {
                let map = process_clouds_1(clouds);
                let cloudy_points = map.iter().filter(|location| location.cloudy()).count();
                print(map);
                println!("Number of cloudy points: {}", cloudy_points);
                assert_eq!(cloudy_points, 5)
            }
            Err(error) => {
                println!("Error: {}", error);
                panic!("Dit not test it at all!");
            }
        }
    }

    #[test]
    fn solution_2() {
        match read_input("../../resources/test-input-dec-5")
        {
            Ok(clouds) => {
                let map = process_clouds_2(clouds);
                let cloudy_points = map.iter().filter(|location| location.cloudy()).count();
                print(map);
                println!("Number of cloudy points: {}", cloudy_points);
                assert_eq!(cloudy_points, 12)
            }
            Err(error) => {
                println!("Error: {}", error);
                panic!("Dit not test it at all!");
            }
        }
    }

    #[test]
    fn iterator() {
        let mut cloud = Cloud {
            from: Location { x: 0, y: 0 },
            to: Location { x: 0, y: 3 },
        };

        let cloud_1 = Cloud {
            from: Location { x: 0, y: 1 },
            to: Location { x: 0, y: 3 },
        };
        let cloud_2 = Cloud {
            from: Location { x: 0, y: 2 },
            to: Location { x: 0, y: 3 },
        };
        let cloud_3 = Cloud {
            from: Location { x: 0, y: 3 },
            to: Location { x: 0, y: 3 },
        };

        assert_eq!(cloud.next().unwrap(), cloud_1);
        assert_eq!(cloud.next().unwrap(), cloud_2);
        assert_eq!(cloud.next().unwrap(), cloud_3);
        assert_eq!(cloud.next(), None);
    }
}
