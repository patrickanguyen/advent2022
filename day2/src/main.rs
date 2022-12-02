use advent::AdventCode;

fn main() {
    let advent = AdventCode::new("day2");

    let lines = match advent.get_input() {
        Ok(lines) => lines,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };

    for line in lines {
        println!("{}", line);
    }
}
