use regex::Regex;
use std::fmt::Formatter;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct Number {
    value: i32,
    chars: String,
}

impl Clone for Number {
    fn clone(&self) -> Self {
        Number {
            value: self.value,
            chars: self.chars.clone(),
        }
    }
}

impl Number {
    fn new(i: i32, s: &str) -> Number {
        Number {
            value: i,
            chars: s.to_string(),
        }
    }

    fn to_regex(&self) -> Regex {
        Regex::new(format!("[{}]", self.chars).as_str()).unwrap()
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.chars)
    }
}

#[derive(Debug)]
struct Display {
    zero: Option<Number>,
    one: Option<Number>,
    two: Option<Number>,
    three: Option<Number>,
    four: Option<Number>,
    five: Option<Number>,
    six: Option<Number>,
    seven: Option<Number>,
    eight: Option<Number>,
    nine: Option<Number>,
}

impl Display {
    fn new() -> Display {
        Display {
            zero: None,
            one: None,
            two: None,
            three: None,
            four: None,
            five: None,
            six: None,
            seven: None,
            eight: None,
            nine: None,
        }
    }

    fn try_regex_count(&self, number: Option<Number>, s: &str) -> Option<i32> {
        if let Some(number) = number {
            return Some(number.to_regex().find_iter(s).count() as i32);
        }
        None
    }

    fn analyze_number(&mut self, number: &str) {
        match number {
            x if x.len() == 2 => {
                self.one = Some(Number::new(1, x));
            }
            x if x.len() == 3 => {
                self.seven = Some(Number::new(7, x));
            }
            x if x.len() == 4 => {
                self.four = Some(Number::new(4, x));
            }
            x if x.len() == 7 => {
                self.eight = Some(Number::new(8, x));
            }
            x => match x {
                x if x.len() == 5 => match x {
                    x if self.try_regex_count(self.seven.clone(), x) == Some(3) => {
                        //7
                        self.three = Some(Number::new(3, x));
                    }
                    x if self.try_regex_count(self.four.clone(), x) == Some(2) => {
                        //4
                        self.two = Some(Number::new(2, x));
                    }
                    x => {
                        self.five = Some(Number::new(5, x));
                    }
                },
                x if x.len() == 6 => match x {
                    x if self.try_regex_count(self.five.clone(), x) == Some(5)
                        && self.try_regex_count(self.one.clone(), x) == Some(1) =>
                    {
                        self.six = Some(Number::new(6, x));
                    }
                    x if self.try_regex_count(self.five.clone(), x) == Some(5) => {
                        self.nine = Some(Number::new(9, x));
                    }
                    x => {
                        self.zero = Some(Number::new(0, x));
                    }
                },
                _ => panic!("Can't parse the number!"),
            },
        };
    }

    fn determine_number(&self, str: &str) -> i32 {
        match str {
            str if self
                .zero
                .clone()
                .unwrap()
                .to_regex()
                .find_iter(str)
                .count()
                .eq(&self.zero.clone().unwrap().chars.len()) && self.zero.clone().unwrap().chars.len() == str.len() =>
            {
                0
            }
            str if self
                .one
                .clone()
                .unwrap()
                .to_regex()
                .find_iter(str)
                .count()
                .eq(&self.one.clone().unwrap().chars.len()) && self.one.clone().unwrap().chars.len() == str.len() =>
            {
                1
            }
            str if self
                .two
                .clone()
                .unwrap()
                .to_regex()
                .find_iter(str)
                .count()
                .eq(&self.two.clone().unwrap().chars.len()) && self.two.clone().unwrap().chars.len() == str.len() =>
            {
                2
            }
            str if self
                .three
                .clone()
                .unwrap()
                .to_regex()
                .find_iter(str)
                .count()
                .eq(&self.three.clone().unwrap().chars.len()) && self.three.clone().unwrap().chars.len() == str.len() =>
            {
                3
            }
            str if self
                .four
                .clone()
                .unwrap()
                .to_regex()
                .find_iter(str)
                .count()
                .eq(&self.four.clone().unwrap().chars.len()) && self.four.clone().unwrap().chars.len() == str.len() =>
            {
                4
            }
            str if self
                .five
                .clone()
                .unwrap()
                .to_regex()
                .find_iter(str)
                .count()
                .eq(&self.five.clone().unwrap().chars.len()) && self.five.clone().unwrap().chars.len() == str.len() =>
            {
                5
            }
            str if self
                .six
                .clone()
                .unwrap()
                .to_regex()
                .find_iter(str)
                .count()
                .eq(&self.six.clone().unwrap().chars.len()) && self.six.clone().unwrap().chars.len() == str.len() =>
            {
                6
            }
            str if self
                .seven
                .clone()
                .unwrap()
                .to_regex()
                .find_iter(str)
                .count()
                .eq(&self.seven.clone().unwrap().chars.len()) && self.seven.clone().unwrap().chars.len() == str.len() =>
            {
                7
            }
            str if self
                .eight
                .clone()
                .unwrap()
                .to_regex()
                .find_iter(str)
                .count()
                .eq(&self.eight.clone().unwrap().chars.len()) && self.eight.clone().unwrap().chars.len() == str.len() =>
            {
                8
            }
            str if self
                .nine
                .clone()
                .unwrap()
                .to_regex()
                .find_iter(str)
                .count()
                .eq(&self.nine.clone().unwrap().chars.len()) && self.nine.clone().unwrap().chars.len() == str.len() =>
            {
                9
            }
            _ => panic!("Could not determine number!"),
        }
    }
}

fn main() {
    if let Ok(mut displays) = read_input("./resources/input-dec-8") {

        let times = displays.clone().into_iter().flatten()
            .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
            .count();
        println!("Digits 1, 4, 7, or 8 appear {} times.", times);

        let mut numbers: Vec<i32> = Vec::new();

        for d in displays.iter_mut() {
            let mut display = Display::new();
            d[..10].sort_by(|x, y| x.len().cmp(&y.len()));
            for n in d[..10].iter() {
                display.analyze_number(n);
            }

            let mut number: Vec<i32> = Vec::new();
            for n in d[11..].iter() {
                let i = display.determine_number(n);
                number.push(i);
            }
            numbers.push(number.iter().fold(0, |acc, elem| acc * 10 + elem));
        }
        println!("Sum of all displays: {}", numbers.iter().sum::<i32>());
    }
}

fn read_input<P>(filename: P) -> io::Result<Vec<Vec<String>>>
where
    P: AsRef<Path>,
{
    let mut displays: Vec<Vec<String>> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    reader.lines().for_each(|line| {
        if let Ok(line) = line {
            let display: Vec<String> = line
                .trim()
                .split_whitespace()
                .map(|x| x.to_string())
                .collect();
            displays.push(display);
        }
    });
    Ok(displays)
}

#[cfg(test)]
mod tests {
    use crate::{read_input, Display, Number};
    use regex::Regex;

    #[test]
    fn solution_2() {
        if let Ok(mut displays) = read_input("./resources/test-input-dec-8") {
            for d in displays.iter_mut() {
                let mut display = Display::new();
                d[..10].sort_by(|x, y| x.len().cmp(&y.len()));
                for n in d[..10].iter() {
                    display.analyze_number(n);
                }
                println!("{:?}", display);

                d[..10].sort_by(|x, y| x.len().cmp(&y.len()));
                for n in d[11..].iter() {
                    let i = display.determine_number(n);
                    println!("{}", i);
                }
            }
        }
    }

    #[test]
    fn regex_count() {
        let test = Display {
            zero: Some(Number {
                value: 0,
                chars: "abcd".to_string(),
            }),
            one: None,
            two: None,
            three: None,
            four: None,
            five: None,
            six: None,
            seven: None,
            eight: None,
            nine: None,
        };

        assert_eq!(
            test.zero.unwrap().to_regex().find_iter("efcdabgh").count(),
            4
        );
    }

    #[test]
    fn create_regex() {
        let zero = Number {
            value: 0,
            chars: "abcd".to_string(),
        };

        assert_eq!(zero.to_regex().find_iter("efcdabgh").count(), 4);
    }

    #[test]
    fn test_regex() {
        let regex = Regex::new(r"([abcd])").unwrap();
        assert_eq!(regex.find_iter("efcdabgh").count(), 4)
    }

    #[test]
    fn test_number_determination() {
        let test = Display {
            zero: Some(Number {
                value: 0,
                chars: "cagedb".to_string(),
            }),
            one: Some(Number {
                value: 1,
                chars: "ab".to_string(),
            }),
            two: None,
            three: None,
            four: None,
            five: None,
            six: None,
            seven: None,
            eight: None,
            nine: Some(Number {
                value: 9,
                chars: "cefabd".to_string(),
            }),
        };

        assert_eq!(test.determine_number("dcbeag"), 0);
        assert_eq!(test.determine_number("ba"), 1);
    }
}
