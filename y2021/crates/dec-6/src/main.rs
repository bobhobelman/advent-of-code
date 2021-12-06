use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

fn main() {
    if let Ok(mut fishes) = read_input("./resources/input-dec-6") {
        for _ in 0..80 {
            let mut new_fishes: Vec<Fish> = Vec::new();
            for fish in fishes.iter_mut() {
                new_fishes.append(&mut fish.night());
            }
            fishes = new_fishes;
        }
        let fishes = fishes.len();
        println!("Fishes: {}", fishes);
    }
}

#[derive(Debug, Clone, Copy)]
struct Fish {
    timer_to_birth: u32,
}

impl Fish {
    fn new() -> Fish {
        Fish { timer_to_birth: 8 }
    }

    fn reset(&mut self) {
        self.timer_to_birth = 6;
    }

    fn night(&mut self) -> Vec<Fish> {
        let mut fishes: Vec<Fish> = Vec::new();
        match self.timer_to_birth {
            0 => {
                self.reset();
                fishes.push(self.clone());
                fishes.push(Fish::new())
            }
            _ => {
                self.timer_to_birth -= 1;
                fishes.push(self.clone());
            }
        }
        fishes
    }
}

impl FromStr for Fish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            timer_to_birth: s.parse::<u32>().unwrap(),
        })
    }
}

fn read_input<P>(filename: P) -> io::Result<Vec<Fish>>
where
    P: AsRef<Path>,
{
    let mut fishes: Vec<Fish> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    reader.lines().for_each(|line| {
        if let Ok(line) = line {
            let mut new_fish: Vec<Fish> = line
                .split(',')
                .map(|s| s.parse::<Fish>().unwrap())
                .collect();
            fishes.append(&mut new_fish);
        }
    });
    Ok(fishes)
}

#[cfg(test)]
mod tests {
    use crate::{read_input, Fish};

    #[test]
    fn solution_1() {
        if let Ok(mut fishes) = read_input("../../resources/test-input-dec-6") {
            for _ in 0..80 {
                let mut new_fishes: Vec<Fish> = Vec::new();
                for fish in fishes.iter_mut() {
                    new_fishes.append(&mut fish.night());
                }
                fishes = new_fishes;
            }
            let fishes = fishes.len();
            assert_eq!(fishes, 5934)
        }
    }
}
