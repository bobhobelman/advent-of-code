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
    fn new(value: i32, chars: &str) -> Number {
        Number {
            value,
            chars: chars.to_string(),
        }
    }

    fn to_regex(&self) -> Regex {
        Regex::new(format!("[{}]", self.chars).as_str()).unwrap()
    }

    fn equal(&self, other: &str) -> bool {
        self.to_regex().find_iter(other).count().eq(&other.len()) && self.chars.len() == other.len()
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

trait Analyzer {
    type Value;
    fn equals(&self, s: &str) -> bool;
    fn count_same_chars(&self, s: &str) -> i32;
}

impl Analyzer for Option<Number> {
    type Value = i32;

    fn equals(&self, other: &str) -> bool {
        self.as_ref().unwrap().equal(other)
    }

    fn count_same_chars(&self, number: &str) -> i32 {
        self.as_ref().unwrap().to_regex().find_iter(number).count() as i32
    }
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
                    x if self.seven.count_same_chars(x) == 3 => {
                        //7
                        self.three = Some(Number::new(3, x));
                    }
                    x if self.four.count_same_chars(x) == 2 => {
                        //4
                        self.two = Some(Number::new(2, x));
                    }
                    x => {
                        self.five = Some(Number::new(5, x));
                    }
                },
                x if x.len() == 6 => match x {
                    x if self.five.count_same_chars(x) == 5
                        && self.one.count_same_chars(x) == 1 =>
                    {
                        self.six = Some(Number::new(6, x));
                    }
                    x if self.five.count_same_chars(x) == 5 => {
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
            str if self.zero.equals(str) => 0,
            str if self.one.equals(str) => 1,
            str if self.two.equals(str) => 2,
            str if self.three.equals(str) => 3,
            str if self.four.equals(str) => 4,
            str if self.five.equals(str) => 5,
            str if self.six.equals(str) => 6,
            str if self.seven.equals(str) => 7,
            str if self.eight.equals(str) => 8,
            str if self.nine.equals(str) => 9,
            _ => panic!("Could not determine number!"),
        }
    }
}

fn main() {
    if let Ok(mut displays) = read_input("./resources/input-dec-8") {
        let times = displays
            .clone()
            .into_iter()
            .flatten()
            .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
            .count();
        println!("Digits 1, 4, 7, or 8 appear {} times.", times);

        let mut numbers: Vec<i32> = Vec::new();

        for d in displays.iter_mut() {
            let mut display = Display::new();

            // Analyze the numbers
            d[..10].sort_by(|x, y| x.len().cmp(&y.len()));
            for n in d[..10].iter() {
                display.analyze_number(n);
            }

            // Get the numbers shown on display
            let mut number: Vec<i32> = Vec::new();
            for n in d[11..].iter() {
                let i = display.determine_number(n);
                number.push(i);
            }

            // Combine the separate digits to one value
            let number = number.iter().fold(0, |acc, elem| acc * 10 + elem);

            numbers.push(number);
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
