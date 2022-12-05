use std::collections::{HashMap, VecDeque};

use advent::AdventCode;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Crates {
    columns: Vec<VecDeque<char>>,
}

impl Crates {
    fn new() -> Crates {
        // Ignore idx 0
        Crates {
            columns: Vec::from([VecDeque::new()]),
        }
    }

    fn fill(&mut self, input: &Vec<String>) {
        // Get indicies of crate columns
        let crate_indicices_row = input.len() - 1;
        let crate_indicies_input = &input[crate_indicices_row];

        // Create new crate column and cache crate column char index
        let mut crate_indicies = HashMap::new();
        for (idx, letter) in crate_indicies_input.char_indices() {
            if !letter.is_whitespace() {
                let col = letter.to_digit(10).expect("Should be an integer") as usize;
                crate_indicies.insert(idx, col);
                self.columns.push(VecDeque::new());
            }
        }

        // Iterate through crate rows except for index row
        for row in input.iter().take(input.len() - 1) {
            for (idx, letter) in row.char_indices() {
                if letter.is_alphabetic() {
                    let col = crate_indicies[&idx];
                    self.columns[col].push_front(letter);
                }
            }
        }
    }

    #[allow(dead_code)]
    fn shift(&mut self, quantity: usize, from_idx: usize, to_idx: usize) {
        for _ in 0..quantity {
            let popped_crate = self.columns[from_idx]
                .pop_back()
                .expect("Column should not be empty");
            self.columns[to_idx].push_back(popped_crate);
        }
    }

    fn shift_keep_order(&mut self, quantity: usize, from_idx: usize, to_idx: usize) {
        let from_col = &mut self.columns[from_idx];
        let split_idx = from_col.len() - quantity;
        let mut move_crates = from_col.split_off(split_idx);

        self.columns[to_idx].append(&mut move_crates);
    }

    fn top_crates(&self) -> String {
        let mut top_crates = String::new();

        for col in &self.columns {
            if let Some(top) = col.back() {
                top_crates.push(*top);
            }
        }

        top_crates
    }
}

fn get_move_vals(input: &String) -> (usize, usize, usize) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^move (?P<quantity>\d+) from (?P<from_idx>\d+) to (?P<to_idx>\d+)$")
                .expect("Regex should compile");
    }

    let captures = RE.captures(&input).expect("Should capture successfully");

    let quantity = captures
        .name("quantity")
        .expect("Should contain quantity")
        .as_str()
        .parse::<usize>()
        .expect("Should be integer");

    let from_idx = captures
        .name("from_idx")
        .expect("Should contain from_idx")
        .as_str()
        .parse::<usize>()
        .expect("Should be integer");

    let to_idx = captures
        .name("to_idx")
        .expect("Should contain to_idx")
        .as_str()
        .parse::<usize>()
        .expect("Should be integer");

    (quantity, from_idx, to_idx)
}

fn main() {
    let advent = AdventCode::new("day5");

    let lines = match advent.get_input() {
        Ok(lines) => lines,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };

    let mut crates = Crates::new();
    let mut crate_input: Vec<String> = Vec::new();
    let mut is_each_crate_initialized = false;

    for line in lines {
        if !is_each_crate_initialized {
            if line.is_empty() {
                crates.fill(&crate_input);
                is_each_crate_initialized = true;
            } else {
                crate_input.push(line);
            }
        } else {
            let (quantity, from_idx, to_idx) = get_move_vals(&line);
            crates.shift_keep_order(quantity, from_idx, to_idx);
        }
    }

    let top_crates = crates.top_crates();
    println!("{}", top_crates);
}
