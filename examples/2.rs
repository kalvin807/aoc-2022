use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn get_score_1(ln: String) -> i32 {
    let mut actions = ln.split_whitespace();
    let opponent = actions.next().unwrap();
    let you = actions.next().unwrap();

    // opponent: A for Rock, B for Paper, and C for Scissors
    // you: X for Rock, Y for Paper, and Z for Scissors
    // point for you: 1 for Rock, 2 for Paper, and 3 for Scissors
    // point for result: 0 if you lost, 3 if the round was a draw, and 6 if you won
    // paper win rock, rock win scissors, scissors win paper
    let mut score = match (opponent, you) {
        ("A", "X") => 3,
        ("A", "Y") => 6,
        ("A", "Z") => 0,
        ("B", "X") => 0,
        ("B", "Y") => 3,
        ("B", "Z") => 6,
        ("C", "X") => 6,
        ("C", "Y") => 0,
        ("C", "Z") => 3,
        _ => 0,
    };

    score = match you {
        "X" => score + 1,
        "Y" => score + 2,
        "Z" => score + 3,
        _ => score,
    };
    score
}

fn get_score_2(ln: String) -> i32 {
    let mut actions = ln.split_whitespace();
    let opponent = actions.next().unwrap();
    let you = actions.next().unwrap();

    // opponent: A for Rock, B for Paper, and C for Scissors
    // you: X for Lost, Y for Draw, and Z for Win
    // point for you: 1 for Rock, 2 for Paper, and 3 for Scissors
    // point for result: 0 if you lost, 3 if the round was a draw, and 6 if you won
    // paper win rock, rock win scissors, scissors win paper
    match you {
        "X" => match opponent {
            "A" => 3,
            "B" => 1,
            "C" => 2,
            _ => 0,
        },
        "Y" => match opponent {
            "A" => 3 + 1,
            "B" => 3 + 2,
            "C" => 3 + 3,
            _ => 0,
        },
        "Z" => match opponent {
            "A" => 6 + 2,
            "B" => 6 + 3,
            "C" => 6 + 1,
            _ => 0,
        },
        _ => 0,
    }
}

fn main() -> std::io::Result<()> {
    let mut path = env::current_dir()?;
    path.push("examples/2");
    println!("reading path: {}", path.display());
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();

    let mut total_1 = 0;
    let mut total_2 = 0;
    for line in lines.flatten() {
        println!("{}", line);
        total_1 += get_score_1(line.clone());
        total_2 += get_score_2(line.clone());
    }

    println!("{}", total_1);
    println!("{}", total_2);

    Ok(())
}
