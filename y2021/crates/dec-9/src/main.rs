use anstyle::Style;
use grid::Grid;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Ord, PartialOrd)]
struct Location {
    height: u32,
    risk: bool,
    x: usize,
    y: usize,
}

impl Location {
    fn new(height: u32) -> Location {
        Location {
            height,
            risk: false,
            x: 99999,
            y: 99999,
        }
    }

    fn risk(&mut self) {
        self.risk = true;
    }

    fn set_coordinate(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    fn part_of_lake(&self) -> bool {
        self.height < 9
    }
}

impl Eq for Location {}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let number = match self.risk {
            false => {
                let green_style = Style::new()
                    .fg_color(Some(anstyle::AnsiColor::Green.into()))
                    .bold();
                let green_style = green_style.render();
                format!("{}{}", green_style, self.height)
            }
            true => {
                let red_style = Style::new()
                    .fg_color(Some(anstyle::AnsiColor::Red.into()))
                    .bold();
                let red_style = red_style.render();
                format!("{}{}", red_style, self.height)
            }
        };
        write!(f, "{}", number)
    }
}

fn print(grid: &Grid<Location>) {
    for row in 0..grid.rows() {
        for number in grid.iter_row(row) {
            print!("{}", number);
        }
        println!();
    }
}

trait Analyzer {
    type Value;
    fn lower(&self, other: Option<&Location>) -> bool;
}

impl Analyzer for Option<&Location> {
    type Value = Location;

    fn lower(&self, other: Option<&Location>) -> bool {
        if let Some(other) = other {
            self.as_ref().unwrap().height < other.height
        } else {
            true
        }
    }
}

fn main() {
    if let Ok(mut map) = read_input("./resources/input-dec-9") {
        // Solution part 1
        for x in 0..map.rows() {
            for y in 0..map.cols() {
                let l = map.get(x, y);
                let n = map.get(x + 1, y);
                let mut s = None;
                if x > 0 {
                    s = map.get(x - 1, y);
                }
                let mut w = None;
                if y > 0 {
                    w = map.get(x, y - 1);
                }
                let e = map.get(x, y + 1);
                if l.lower(n) && l.lower(s) && l.lower(w) && l.lower(e) {
                    let location = map.get_mut(x, y).unwrap();
                    location.risk();
                    location.set_coordinate(x, y);
                }
            }
        }
        print(&map);

        let lowest_points: Vec<&Location> = map.iter().filter(|x| x.risk).collect();
        let sum = lowest_points.iter().map(|x| x.height + 1).sum::<u32>();
        println!("Sum of risk levels: {}", sum);

        // Solution part 2
        let mut lake_sizes: Vec<i32> = Vec::new();
        for l in lowest_points.iter() {
            let mut lake: Vec<Location> = vec![Location {
                height: l.height,
                risk: l.risk,
                x: l.x,
                y: l.y,
            }];
            let mut cont = true;

            while cont {
                let mut new_discovered_lake: Vec<Location> = Vec::new();
                for p in lake.iter() {
                    if let Some(location) = map.get(p.x + 1, p.y) {
                        let new_location = Location {
                            height: location.height,
                            risk: false,
                            x: p.x + 1,
                            y: p.y,
                        };
                        if location.part_of_lake() && !lake.contains(&new_location) {
                            new_discovered_lake.push(new_location)
                        }
                    }
                    if p.x > 0 {
                        if let Some(location) = map.get(p.x - 1, p.y) {
                            let new_location = Location {
                                height: location.height,
                                risk: false,
                                x: p.x - 1,
                                y: p.y,
                            };
                            if location.part_of_lake() && !lake.contains(&new_location) {
                                new_discovered_lake.push(new_location)
                            }
                        }
                    }
                    if p.y > 0 {
                        if let Some(location) = map.get(p.x, p.y - 1) {
                            let new_location = Location {
                                height: location.height,
                                risk: false,
                                x: p.x,
                                y: p.y - 1,
                            };
                            if location.part_of_lake() && !lake.contains(&new_location) {
                                new_discovered_lake.push(new_location)
                            }
                        }
                    }
                    if let Some(location) = map.get(p.x, p.y + 1) {
                        let new_location = Location {
                            height: location.height,
                            risk: false,
                            x: p.x,
                            y: p.y + 1,
                        };
                        if location.part_of_lake() && !lake.contains(&new_location) {
                            new_discovered_lake.push(new_location)
                        }
                    }
                }
                if new_discovered_lake.is_empty() {
                    cont = false;
                }
                lake.append(&mut new_discovered_lake);
            }
            lake.sort();
            lake.dedup();
            lake_sizes.push(lake.len() as i32)
        }
        lake_sizes.sort_unstable();
        println!(
            "3 largest lakes multiplied: {}",
            lake_sizes.pop().unwrap() * lake_sizes.pop().unwrap() * lake_sizes.pop().unwrap()
        );
    }
}

fn read_input<P>(filename: P) -> io::Result<Grid<Location>>
where
    P: AsRef<Path>,
{
    let mut map: Vec<Vec<Location>> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    reader.lines().for_each(|line| {
        if let Ok(line) = line {
            map.push(
                line.trim()
                    .chars()
                    .map(|c| Location::new(c.to_digit(10).unwrap() as u32))
                    .collect(),
            )
        }
    });
    let cols = map[0].len();
    let map: Grid<Location> = Grid::from_vec(map.into_iter().flatten().into_iter().collect(), cols);

    Ok(map)
}
