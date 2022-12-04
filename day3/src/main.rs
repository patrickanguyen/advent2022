use advent::AdventCode;

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

    for group in lines.chunks(3) {
        let mut inter = group[0]
            .chars()
            .filter(|item| group[1].contains(*item))
            .filter(|item| group[2].contains(*item));

        let same = inter
            .next()
            .expect("Intersection should contain at least 1 element");

        sum += get_priority(&same);
    }

    println!("Sum: {}", sum);
}
