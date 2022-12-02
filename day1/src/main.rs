use clap::{App, Arg};
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let matches = App::new("Day1")
        .arg(Arg::new("input_path").value_name("FILE").required(true))
        .get_matches();

    let reader = {
        let input_path = matches
            .value_of("input_path")
            .expect("Argument should always be given");

        let input = match File::open(input_path) {
            Ok(f) => f,
            Err(_) => panic!("Unable to open file"),
        };

        BufReader::new(input)
    };

    let lines = reader.lines().map(|line| match line {
        Ok(line) => line,
        Err(err) => {
            println!("Error: Unable to read file - {}", err);
            std::process::exit(1);
        }
    });

    let mut current_calories = 0;
    let mut calories_heap = BinaryHeap::new();

    for line in lines {
        if line.is_empty() {
            println!("Current Elf Calories: {}", current_calories);
            calories_heap.push(current_calories);
            current_calories = 0;
        } else {
            let calories: u32 = line.parse().expect("Should successfully parse to u32");
            current_calories += calories;
        }
    }
    calories_heap.push(current_calories);

    let mut sum = 0;
    for _ in 0..3 {
        sum += calories_heap.pop().unwrap();
    }

    println!("Top 3 Sum: {}", sum)
}
