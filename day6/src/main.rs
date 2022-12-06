use advent::AdventCode;

fn is_each_char_unique(input: &str) -> bool {
    debug_assert!(input
        .chars()
        .all(|letter| letter.is_ascii() && letter.is_alphabetic() && letter.is_lowercase()));

    let mut bitmap: u32 = 0;
    let base = 'a' as u32;

    for letter in input.chars() {
        // Check in bitmap if letter already exists
        let idx = (letter as u32) - base;

        if (bitmap >> idx) & 1 == 1 {
            return false;
        }
        bitmap |= 1 << idx;
    }

    true
}

fn find_marker(data_stream: &String, num_unique: usize) -> Option<usize> {
    for idx in 0..(data_stream.len() - num_unique) {
        let word = &data_stream[idx..(idx + num_unique)];
        if is_each_char_unique(word) {
            return Some(idx + num_unique);
        }
    }
    None
}

fn main() {
    let advent = AdventCode::new("day6");

    let lines = match advent.get_input() {
        Ok(lines) => lines,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };

    const START_PACKET_COUNT: usize = 4;
    const START_MSG_PACKET_COUNT: usize = 14;

    for line in lines {
        match find_marker(&line, START_PACKET_COUNT) {
            Some(marker) => println!("Marker = {}", marker),
            None => println!("No marker found"),
        }

        match find_marker(&line, START_MSG_PACKET_COUNT) {
            Some(marker) => println!("Start Marker = {}", marker),
            None => println!("No start marker found"),
        }
    }
}
