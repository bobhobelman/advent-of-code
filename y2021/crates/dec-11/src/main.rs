use anstyle::Style;
use grid::Grid;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

enum Synced {
    No { flashes: u32 },
    Yes { flashes: u32, synced_step: u32 },
}

impl Synced {
    fn flashes(&self) -> u32 {
        match self {
            Synced::No { flashes } => *flashes,
            Synced::Yes {
                flashes,
                synced_step: _,
            } => *flashes,
        }
    }

    fn step_in_sync(&self) -> Option<u32> {
        match self {
            Synced::No { flashes: _ } => None,
            Synced::Yes {
                flashes: _,
                synced_step,
            } => Some(*synced_step),
        }
    }
}

#[derive(Clone)]
struct Octopus {
    energy: u32,
    flash: bool,
    flashes: u32,
}

impl Octopus {
    fn new(energy: u32) -> Octopus {
        Octopus {
            energy,
            flash: false,
            flashes: 0,
        }
    }

    fn step(&mut self) -> bool {
        if !self.flash {
            self.energy += 1;

            if self.energy > 9 {
                self.flash = true;
                self.flashes += 1;
                return true;
            }
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
        let number = match self.energy {
            0 => {
                let zero_style = Style::new()
                    .fg_color(Some(anstyle::AnsiColor::Yellow.into()))
                    .bold();
                let zero_style = zero_style.render();
                format!("{}{}", zero_style, self.energy)
            }
            _ => {
                let none_style = Style::new()
                    .fg_color(Some(anstyle::AnsiColor::Blue.into()))
                    .bold();
                let none_style = none_style.render();
                format!("{}{}", none_style, self.energy)
            }
        };
        write!(f, "{}", number)
    }
}

trait OctopusGrid {
    fn print(&self);
    fn step_neighbours(&mut self, x: usize, y: usize);
    fn step(&mut self, x: usize, y: usize);
    fn process_steps(&mut self, steps: u32) -> Synced;
    fn in_sync(&self) -> bool;
    fn total_flashes(&self) -> u32;
    fn steps_to_get_in_sync(&mut self) -> u32;
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
            self.step(x, y);
        }
        if let (Some(x), Some(y)) = (x.checked_add(1), Some(y)) {
            self.step(x, y);
        }
        if let (Some(x), Some(y)) = (x.checked_add(1), y.checked_add(1)) {
            self.step(x, y);
        }
        if let (Some(x), Some(y)) = (Some(x), y.checked_sub(1)) {
            self.step(x, y);
        }
        if let (Some(x), Some(y)) = (Some(x), y.checked_add(1)) {
            self.step(x, y);
        }
        if let (Some(x), Some(y)) = (x.checked_sub(1), y.checked_sub(1)) {
            self.step(x, y);
        }
        if let (Some(x), Some(y)) = (x.checked_sub(1), Some(y)) {
            self.step(x, y);
        }
        if let (Some(x), Some(y)) = (x.checked_sub(1), y.checked_add(1)) {
            self.step(x, y);
        }
    }

    fn step(&mut self, x: usize, y: usize) {
        if let Some(octopus) = self.get_mut(x, y) {
            if octopus.step() {
                self.step_neighbours(x, y);
            }
        }
    }

    fn process_steps(&mut self, steps: u32) -> Synced {
        let mut synced_step: Option<u32> = None;

        for i in 0..steps {
            for x in 0..self.rows() {
                for y in 0..self.cols() {
                    self.step(x, y);
                }
            }
            self.iter_mut().filter(|o| o.flash).for_each(|o| o.reset());

            if self.in_sync() && synced_step.is_none() {
                synced_step = Some(i + 1);
            }
        }

        let flashes = self.total_flashes();

        if let Some(synced_step) = synced_step {
            Synced::Yes {
                flashes,
                synced_step,
            }
        } else {
            Synced::No { flashes }
        }
    }

    fn in_sync(&self) -> bool {
        self.iter()
            .all(|x| x.energy == self.get(0, 0).unwrap().energy)
    }

    fn total_flashes(&self) -> u32 {
        self.iter().map(|x| x.flashes).sum::<u32>()
    }

    fn steps_to_get_in_sync(&mut self) -> u32 {
        let mut steps = 0;
        while !self.in_sync() {
            for x in 0..self.rows() {
                for y in 0..self.cols() {
                    self.step(x, y);
                }
            }
            self.iter_mut().filter(|o| o.flash).for_each(|o| o.reset());
            steps += 1;
            if steps > 100_000 {
                panic!("won't get in sync")
            }
        }
        steps
    }
}

fn main() {
    if let Ok(cavern) = read_input("./resources/input-dec-11") {
        println!("Solution 1:");
        let mut cavern: Grid<Octopus> = cavern;
        let result = cavern.process_steps(100);
        println!("Total flashes: {}", result.flashes());
        cavern.print();
    }

    if let Ok(cavern) = read_input("./resources/input-dec-11") {
        println!("\nSolution 2a:");
        let mut cavern: Grid<Octopus> = cavern;
        let result = cavern.process_steps(1000);
        println!("Total flashes: {}", cavern.total_flashes());
        if let Some(step) = result.step_in_sync() {
            println!("Step {} all are pulsing at same time.", step);
        }
    }

    if let Ok(cavern) = read_input("./resources/input-dec-11") {
        println!("\nSolution 2b:");
        let mut cavern: Grid<Octopus> = cavern;
        let in_sync_step = cavern.steps_to_get_in_sync();
        println!("Total flashes: {}", cavern.total_flashes());
        println!("Step {} all are pulsing at same time.", in_sync_step);
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
                    .map(|c| Octopus::new(c.to_digit(10).unwrap() as u32))
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
    use crate::OS;
    use crate::{read_input, OctopusGrid};

    #[test]
    fn solution_1() {
        if let Ok(mut cavern) = read_input("../../resources/test-input-dec-11") {
            let result = cavern.process_steps(100);

            assert_eq!(result.flashes(), 1656);
        } else {
            panic!("Fail!")
        }
    }

    #[test]
    fn solution_2a() {
        if let Ok(mut cavern) = read_input("../../resources/test-input-dec-11") {
            let result = cavern.process_steps(1000);
            assert_eq!(result.step_in_sync().unwrap(), 195);
        } else {
            panic!("Fail!")
        }
    }

    #[test]
    fn solution_2b() {
        if let Ok(mut cavern) = read_input("../../resources/test-input-dec-11") {
            let result = cavern.steps_to_get_in_sync();
            assert_eq!(result, 195);
        } else {
            panic!("Fail!")
        }
    }

    #[test]
    fn debian() {
        assert_eq!(OS::DEBIAN.eq("KUT"), true);
    }
}
