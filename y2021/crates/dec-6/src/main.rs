use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

fn main() {
    if let Ok(fishes) = read_input("./resources/input-dec-6") {
        let mut fishes_counted: u64 = 0;
        let mut precounted: HashMap<u8, u64> = HashMap::new();
        for fish in fishes {
            let fishes = precounted.entry(fish.timer_to_birth).or_insert(fish.night_quick(256));
            fishes_counted += *fishes
        }
        println!("Fishes: {}", fishes_counted)
    }
}

#[derive(Debug, Clone, Copy)]
struct Fish {
    timer_to_birth: u8,
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

    fn night_quick(&self, nights: u64) -> u64 {
        let mut fishes = vec![*self];
        for _ in 0..nights {
            let mut new_fishes: Vec<Fish>= Vec::new();
            for fish in fishes.iter_mut() {
                new_fishes.append(&mut fish.night());
            }
            fishes = new_fishes;
        }
        fishes.len() as u64
    }

    fn quick_256_night(&mut self) -> u64 {
        match self.timer_to_birth {
            1 => 6206821033,
            2 => 5617089148,
            3 => 5217223242,
            4 => 4726100874,
            5 => 4368232009,
            _ => panic!("Wrong number")
        }
    }
}

impl FromStr for Fish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            timer_to_birth: s.parse::<u8>().unwrap(),
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
    use std::collections::HashMap;
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

    #[test]
    fn solution_2() {
        if let Ok(fishes) = read_input("../../resources/test-input-dec-6") {
            let mut fishes_counted: u64 = 0;
                for mut fish in fishes {
                    fishes_counted += fish.quick_256_night();
                }
            assert_eq!(fishes_counted, 26984457539)
        }
    }

    #[test]
    fn solution_1_new() {
        if let Ok(fishes) = read_input("../../resources/test-input-dec-6") {
            let mut fishes_counted: u64 = 0;
            let mut precounted: HashMap<u8, u64> = HashMap::new();
            for fish in fishes {
                let fishes = precounted.entry(fish.timer_to_birth).or_insert(fish.night_quick(80));
                fishes_counted += *fishes
            }
            assert_eq!(fishes_counted, 5934)
        }
    }
}
