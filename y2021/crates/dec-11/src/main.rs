use ansi_term::{Colour, Style};
use grid::Grid;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

enum Synced {
    No(u64),
    Yes(u64, u32),
}

impl Synced {
    fn flashes(&self) -> u64 {
        match self {
            Synced::No(f) => *f,
            Synced::Yes(f, _) => *f,
        }
    }

    fn step_in_sync(&self) -> Option<u32> {
        match self {
            Synced::No(_) => None,
            Synced::Yes(_, s) => Some(*s),
        }
    }
}

#[derive(Clone)]
struct Octopus {
    energy: u64,
    flash: bool,
    flashes: u64,
}

impl Octopus {
    fn new(energy: u64) -> Octopus {
        Octopus {
            energy,
            flash: false,
            flashes: 0,
        }
    }

    fn step(&mut self) -> bool {
        if !self.flash {
            self.energy += 1;
        }
        if self.energy > 9 && !self.flash {
            self.flash = true;
            self.flashes += 1;
            return true;
        }
        false
    }

    fn reset(&mut self) {
        self.energy = 0;
        self.flash = false;
    }
}

impl Display for Octopus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let number = match self.flash {
            false => Style::new()
                .fg(Colour::Blue)
                .bold()
                .paint(format!("{}", self.energy)),
            true => Style::new()
                .fg(Colour::Yellow)
                .bold()
                .paint(format!("{}", self.energy)),
        };
        write!(f, "{}", number)
    }
}

trait OctopusGrid {
    fn print(&self);
    fn step_neighbours(&mut self, x: usize, y: usize);
    fn neighbour_step(&mut self, x: usize, y: usize);
}

impl OctopusGrid for Grid<Octopus> {
    fn print(&self) {
        for row in 0..self.rows() {
            for octopus in self.iter_row(row) {
                print!("{}", octopus);
            }
            println!()
        }
    }

    fn step_neighbours(&mut self, x: usize, y: usize) {
        if let (Some(x), Some(y)) = (x.checked_add(1), y.checked_sub(1)) {
            self.neighbour_step(x, y);
        }
        if let (Some(x), Some(y)) = (x.checked_add(1), Some(y)) {
            self.neighbour_step(x, y);
        }
        if let (Some(x), Some(y)) = (x.checked_add(1), y.checked_add(1)) {
            self.neighbour_step(x, y);
        }
        if let (Some(x), Some(y)) = (Some(x), y.checked_sub(1)) {
            self.neighbour_step(x, y);
        }
        if let (Some(x), Some(y)) = (Some(x), y.checked_add(1)) {
            self.neighbour_step(x, y);
        }
        if let (Some(x), Some(y)) = (x.checked_sub(1), y.checked_sub(1)) {
            self.neighbour_step(x, y);
        }
        if let (Some(x), Some(y)) = (x.checked_sub(1), Some(y)) {
            self.neighbour_step(x, y);
        }
        if let (Some(x), Some(y)) = (x.checked_sub(1), y.checked_add(1)) {
            self.neighbour_step(x, y);
        }
    }

    fn neighbour_step(&mut self, x: usize, y: usize) {
        if let Some(octopus) = self.get_mut(x, y) {
            if octopus.step() {
                self.step_neighbours(x, y);
            }
        }
    }
}

fn process_steps(mut cavern: Grid<Octopus>, steps: u32) -> Synced {
    let mut synced_step = 0;
    // cavern.print();
    for i in 0..steps {
        for x in 0..cavern.rows() {
            for y in 0..cavern.cols() {
                if let Some(octopus) = cavern.get_mut(x, y) {
                    if octopus.step() {
                        cavern.step_neighbours(x, y);
                    }
                }
            }
        }
        // cavern.print();
        cavern
            .iter_mut()
            .filter(|o| o.flash)
            .for_each(|o| o.reset());
        if cavern
            .iter()
            .all(|x| x.energy == cavern.get(0, 0).unwrap().energy)
            && synced_step == 0
        {
            synced_step = i + 1;
        }
    }
    let flashes = cavern.iter().map(|x| x.flashes).sum::<u64>();
    if synced_step == 0 {
        Synced::No(flashes)
    } else {
        Synced::Yes(flashes, synced_step)
    }
}

fn main() {
    if let Ok(cavern) = read_input("./resources/input-dec-11") {
        println!("Solution 1:");
        let result = process_steps(cavern.clone(), 100);
        println!("Total flashes: {}", result.flashes());

        println!("Solution 2:");
        let result = process_steps(cavern, 1000);
        println!("Total flashes: {}", result.flashes());
        if let Some(sync_moment) = result.step_in_sync() {
            println!("Step {} all are pulsing at same time.", sync_moment);
        }
    }
}

fn read_input<P>(filename: P) -> io::Result<Grid<Octopus>>
where
    P: AsRef<Path>,
{
    let mut map: Vec<Vec<Octopus>> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    reader.lines().for_each(|line| {
        if let Ok(line) = line {
            map.push(
                line.trim()
                    .chars()
                    .map(|c| Octopus::new(c.to_digit(10).unwrap() as u64))
                    .collect(),
            )
        }
    });
    let cols = map[0].len();
    let map: Grid<Octopus> = Grid::from_vec(map.into_iter().flatten().into_iter().collect(), cols);

    Ok(map)
}

#[cfg(test)]
mod tests {
    use crate::{process_steps, read_input};

    #[test]
    fn solution_1() {
        if let Ok(cavern) = read_input("../../resources/test-input-dec-11") {
            let result = process_steps(cavern, 100);

            assert_eq!(result.flashes(), 1656);
        } else {
            panic!("Fail!")
        }
    }

    #[test]
    fn solution_2() {
        if let Ok(cavern) = read_input("../../resources/test-input-dec-11") {
            let result = process_steps(cavern, 1000);
            assert_eq!(result.step_in_sync(), Some(195));
        } else {
            panic!("Fail!")
        }
    }
}
