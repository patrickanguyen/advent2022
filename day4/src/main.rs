use std::{cmp::Ordering, fmt};

use advent::AdventCode;
use regex::Regex;

#[derive(Debug)]
struct Elf {
    lower: u32,
    upper: u32,
}

impl Elf {
    fn fully_overlaps(&self, other: &Elf) -> bool {
        match self.lower.cmp(&other.lower) {
            Ordering::Less => self.upper >= other.upper,
            Ordering::Equal => true,
            Ordering::Greater => other.upper >= self.upper,
        }
    }

    fn overlap(&self, other: &Elf) -> bool {
        match self.lower.cmp(&other.lower) {
            Ordering::Less => self.upper >= other.lower,
            Ordering::Equal => true,
            Ordering::Greater => other.upper >= self.lower,
        }
    }
}

impl fmt::Display for Elf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}-{})", self.lower, self.upper)
    }
}

fn main() {
    let advent = AdventCode::new("day4");

    let lines = match advent.get_input() {
        Ok(lines) => lines,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };

    let mut fully_overlaps = 0;
    let mut overlaps = 0;
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").expect("Regex should successfully compile");

    for line in lines {
        let cap = re
            .captures(&line)
            .expect("Input should be successfully captured by the regex");

        let elf1 = Elf {
            lower: cap[1].parse().unwrap(),
            upper: cap[2].parse().unwrap(),
        };
        let elf2 = Elf {
            lower: cap[3].parse().unwrap(),
            upper: cap[4].parse().unwrap(),
        };

        let is_fully_overlaping = elf1.fully_overlaps(&elf2);
        let is_overlaping = elf1.overlap(&elf2);

        if is_overlaping {
            overlaps += 1;
        }

        if is_fully_overlaping {
            fully_overlaps += 1;
        }
    }

    println!("Fully Overlaps: {}", fully_overlaps);
    println!("Overlaps: {}", overlaps);
}
