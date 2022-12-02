use advent::AdventCode;

enum Outcome {
    Win,
    Lost,
    Draw,
}

impl Outcome {
    fn value(&self) -> u32 {
        match *self {
            Outcome::Win => 6,
            Outcome::Lost => 0,
            Outcome::Draw => 3,
        }
    }

    fn get_player_move(&self, opponent: Hand) -> Hand {
        match opponent {
            Hand::Rock => match *self {
                Outcome::Win => Hand::Paper,
                Outcome::Lost => Hand::Scissors,
                Outcome::Draw => Hand::Rock,
            },
            Hand::Paper => match *self {
                Outcome::Win => Hand::Scissors,
                Outcome::Lost => Hand::Rock,
                Outcome::Draw => Hand::Paper,
            },
            Hand::Scissors => match *self {
                Outcome::Win => Hand::Rock,
                Outcome::Lost => Hand::Paper,
                Outcome::Draw => Hand::Scissors,
            },
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Lost),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err("Invalid input"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn value(&self) -> u32 {
        match *self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    #[allow(dead_code)]
    fn play(&self, opponent: Hand) -> Outcome {
        match *self {
            Hand::Rock => match opponent {
                Hand::Rock => Outcome::Draw,
                Hand::Paper => Outcome::Lost,
                Hand::Scissors => Outcome::Win,
            },
            Hand::Paper => match opponent {
                Hand::Rock => Outcome::Win,
                Hand::Paper => Outcome::Draw,
                Hand::Scissors => Outcome::Lost,
            },
            Hand::Scissors => match opponent {
                Hand::Rock => Outcome::Lost,
                Hand::Paper => Outcome::Win,
                Hand::Scissors => Outcome::Draw,
            },
        }
    }
}

impl TryFrom<char> for Hand {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Hand::Rock),
            'B' | 'Y' => Ok(Hand::Paper),
            'C' | 'Z' => Ok(Hand::Scissors),
            _ => Err("Invalid input"),
        }
    }
}

fn main() {
    let advent = AdventCode::new("day2");

    let lines = match advent.get_input() {
        Ok(lines) => lines,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };

    let mut total_points = 0;

    for line in lines {
        let char1 = line.chars().next().unwrap();
        let char2 = line.chars().nth(2).unwrap();

        let opponent: Hand = char1.try_into().unwrap();
        let outcome: Outcome = char2.try_into().unwrap();

        let player = outcome.get_player_move(opponent);
        total_points += player.value() + outcome.value();
    }

    println!("Total Points: {}", total_points);
}
