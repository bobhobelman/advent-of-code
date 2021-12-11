use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

enum Score {
    Error(i32),
    Complete(u64),
}

fn check_format(chunks: String) -> Score {
    let mut chars: Vec<char> = Vec::new();

    for c in chunks.chars() {
        match c {
            '(' => {
                chars.push(c);
            }
            ')' => {
                let last = chars.pop().unwrap();
                if !last.eq(&'(') {
                    println!("Expected: ( got: {}", last);
                    return Score::Error(3);
                }
            }
            '[' => {
                chars.push(c);
            }
            ']' => {
                let last = chars.pop().unwrap();
                if !last.eq(&'[') {
                    println!("Expected: [ got: {}", last);
                    return Score::Error(57);
                }
            }
            '{' => {
                chars.push(c);
            }
            '}' => {
                let last = chars.pop().unwrap();
                if !last.eq(&'{') {
                    println!("Expected: {{ got: {}", last);
                    return Score::Error(1197);
                }
            }
            '<' => {
                chars.push(c);
            }
            '>' => {
                let last = chars.pop().unwrap();
                if !last.eq(&'<') {
                    println!("Expected: < got: {}", last);
                    return Score::Error(25137);
                }
            }
            x => {
                println!("Did not expect this: {}", x);
                return Score::Error(0);
            }
        }
    }
    let mut score = 0u64;
    chars.reverse();
    for c in chars {
        score *= 5;
        match c {
            '(' => score += 1,
            '[' => score += 2,
            '{' => score += 3,
            '<' => score += 4,
            x => {
                println!("Did not expect this: {}", x)
            }
        }
    }
    Score::Complete(score)
}

fn main() {
    if let Ok(lines) = read_input("./resources/input-dec-10") {
        let mut score = 0;
        let mut complete_scores: Vec<u64> = Vec::new();
        for line in lines {
            match check_format(line) {
                Score::Error(s) => score += s,
                Score::Complete(s) => complete_scores.push(s),
            }
        }
        println!("Error score: {}", score);
        complete_scores.sort_unstable();
        println!(
            "Complete score: {}",
            complete_scores.get(complete_scores.len() / 2).unwrap()
        );
    }
}

fn read_input<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let mut displays: Vec<String> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    reader.lines().for_each(|line| {
        if let Ok(line) = line {
            displays.push(line.trim().to_string());
        }
    });
    Ok(displays)
}

#[cfg(test)]
mod tests {
    use crate::{check_format, read_input, Score};

    #[test]
    fn solution_1() {
        if let Ok(lines) = read_input("../../test/resources/input-dec-10") {
            let mut error_score = 0;
            let mut complete_scores: Vec<u64> = Vec::new();
            for line in lines {
                match check_format(line) {
                    Score::Error(s) => error_score += s,
                    Score::Complete(s) => complete_scores.push(s),
                }
            }
            assert_eq!(error_score, 26397);
        }
    }

    #[test]
    fn solution_2() {
        if let Ok(lines) = read_input("../../test/resources/input-dec-10") {
            let mut error_score = 0;
            let mut complete_scores: Vec<u64> = Vec::new();
            for line in lines {
                match check_format(line) {
                    Score::Error(s) => error_score += s,
                    Score::Complete(s) => complete_scores.push(s),
                }
            }
            complete_scores.sort_unstable();
            let complete_scores = *complete_scores.get(complete_scores.len() / 2).unwrap();
            assert_eq!(error_score, 26397);
            assert_eq!(complete_scores, 288957u64);
        }
    }
}
