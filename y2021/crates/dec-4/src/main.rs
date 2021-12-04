use ansi_term::Style;
use grid::*;
use io::BufReader;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::StepBy;
use std::path::Path;
use std::slice::Iter;
use std::str::FromStr;

#[derive(Eq, PartialEq)]
enum Puzzle {
    One,
    Two,
}

#[derive(Clone, Copy, Debug)]
struct Number {
    value: i32,
    marked: bool,
}

impl FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            value: s.parse::<i32>().unwrap(),
            marked: false,
        })
    }
}

impl Default for Number {
    fn default() -> Self {
        Self {
            value: 0,
            marked: false,
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let number = match self.marked {
            true => Style::new().bold().paint(format!("{:02}", self.value)),
            false => Style::new().paint(format!("{:02}", self.value)),
        };
        write!(f, "{} ", number)
    }
}

#[derive(Clone)]
struct Card {
    field: Grid<Number>,
    bingo: bool,
}

impl Card {
    fn new(rows: usize, cols: usize) -> Card {
        Card {
            field: Grid::new(rows, cols),
            bingo: false,
        }
    }

    fn push_row(&mut self, row: Vec<Number>) {
        self.field.push_row(row)
    }

    fn rows(&self) -> usize {
        self.field.rows()
    }

    fn cols(&self) -> usize {
        self.field.cols()
    }

    fn iter_col(&self, i: usize) -> StepBy<Iter<Number>> {
        self.field.iter_col(i)
    }

    fn iter_row(&self, i: usize) -> Iter<Number> {
        self.field.iter_row(i)
    }

    fn clear(&mut self) {
        self.field.clear();
    }

    fn is_empty(&self) -> bool {
        self.field.is_empty()
    }

    fn had_bingo(&self) -> bool {
        self.bingo
    }

    fn bingo(&mut self) {
        self.bingo = true;
    }

    fn print(&self) {
        for row in 0..self.field.rows() {
            for number in self.field.iter_row(row) {
                print!("{}", number);
            }
            println!();
        }
    }

    fn calculate_print_winner(&self, number: &i32) -> i32 {
        println!("Winner!");
        self.print();
        let result = self.sum_unmarked();
        println!(
            "Number: {}, Sum: {}, Answer: {}",
            number,
            result,
            number * result
        );
        number * result
    }

    fn sum_unmarked(&self) -> i32 {
        self.field
            .iter()
            .filter(|n| !n.marked)
            .map(|n| n.value)
            .sum()
    }
}

fn main() {
    if let Ok((numbers, cards)) = read_input("./resources/input-dec-4") {
        let bingo = play_bingo(numbers, cards, Puzzle::One);
        println!("Bingo: {}", bingo);
    }

    if let Ok((numbers, cards)) = read_input("./resources/input-dec-4") {
        let bingo = play_bingo(numbers, cards, Puzzle::Two);
        println!("Last Bingo: {}", bingo);
    }
}

fn play_bingo(numbers: Vec<i32>, mut cards: Vec<Card>, puzzle: Puzzle) -> i32 {
    let mut last_card = false;
    for number in numbers.iter() {
        cards.retain(|card| !card.had_bingo());
        if cards.len() == 1 {
            last_card = true;
        }

        let mut check_card = false;
        for card in cards.iter_mut() {
            for card_number in card.field.iter_mut() {
                if card_number.value.eq(number) {
                    card_number.marked = true;
                    check_card = true;
                }
            }
            if check_card {
                check_card = false;
                for row in 0..card.rows() {
                    if card.iter_row(row).filter(|x| x.marked.eq(&true)).count() == 5 {
                        card.bingo();
                        if last_card || puzzle == Puzzle::One {
                            return card.calculate_print_winner(number);
                        }
                    }
                }
                for col in 0..card.cols() {
                    if card.iter_col(col).filter(|x| x.marked.eq(&true)).count() == 5 {
                        card.bingo();
                        if last_card || puzzle == Puzzle::One {
                            return card.calculate_print_winner(number);
                        }
                    }
                }
            }
        }
    }
    panic!("No winner found!")
}

fn read_input<P>(filename: P) -> io::Result<(Vec<i32>, Vec<Card>)>
where
    P: AsRef<Path>,
{
    let mut numbers: Vec<i32> = Vec::new();
    let mut cards: Vec<Card> = Vec::new();

    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut numbers_input: String = String::new();
    if reader.read_line(&mut numbers_input).is_ok() {
        numbers = numbers_input
            .trim()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
    }

    let mut card: Card = Card::new(5, 5);
    card.clear();
    reader.lines().for_each(|line| {
        if let Ok(line) = line {
            match line.trim() {
                x if x.is_empty() => {
                    if !card.is_empty() {
                        cards.push(card.clone());
                    }
                    card.clear();
                }
                x => {
                    let row: Vec<Number> = x
                        .split_whitespace()
                        .map(|x| x.parse::<Number>().unwrap())
                        .collect();
                    card.push_row(row);
                }
            }
        }
    });
    Ok((numbers, cards))
}

#[cfg(test)]
mod tests {
    use crate::{play_bingo, read_input, Puzzle};

    #[test]
    fn test_1() {
        let expected = 4512;
        match read_input("./resources/test-input-dec-4") {
            Ok((numbers, cards)) => {
                let bingo = play_bingo(numbers, cards, Puzzle::One);
                assert_eq!(
                    bingo, expected,
                    "Response: {}, should be: {}",
                    bingo, expected
                )
            }
            Err(error) => {
                println!("{}", error)
            }
        }
    }

    #[test]
    fn test_2() {
        let expected = 1924;
        match read_input("./resources/test-input-dec-4") {
            Ok((numbers, cards)) => {
                let bingo = play_bingo(numbers, cards, Puzzle::Two);
                assert_eq!(
                    bingo, expected,
                    "Response: {}, should be: {}",
                    bingo, expected
                )
            }
            Err(error) => {
                println!("{}", error)
            }
        }
    }
}
