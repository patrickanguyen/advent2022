use advent::AdventCode;
use std::collections::BinaryHeap;

fn main() {
    let advent = AdventCode::new("day1");

    let lines = match advent.get_input() {
        Ok(lines) => lines,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };

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
