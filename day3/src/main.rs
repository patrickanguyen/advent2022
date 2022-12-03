use advent::AdventCode;
use std::collections::HashSet;

fn get_priority(item: &char) -> u32 {
    let val = *item as u32;

    if item.is_uppercase() {
        let base = 'A' as u32;
        val - base + 27
    } else {
        let base = 'a' as u32;
        val - base + 1
    }
}

fn main() {
    let advent = AdventCode::new("day3");

    let lines = match advent.get_input() {
        Ok(lines) => lines,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };

    let mut sum = 0;
    let mut sets = Vec::new();
    sets.reserve(3);

    for line in lines {
        let set: HashSet<char> = line.chars().collect();
        sets.push(set);

        if sets.len() == 3 {
            let mut inter = sets[0].iter()
                .filter(|item| sets[1].contains(item))
                .filter(|item| sets[2].contains(item));

            let same = inter.next().expect("Intersection should contain at least 1 element");
            let priority = get_priority(same);

            sum += priority;
            sets.clear();
        }
    }

    println!("Sum: {}", sum);
}
