use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Read, Seek, SeekFrom};

fn get_score(c: char) -> i32 {
    match c.is_ascii_lowercase() {
        true => c as i32 - 96,
        false => c as i32 - 64 + 26,
    }
}

fn find_misplaced(ln: &str) -> char {
    let midpoint = ln.len() / 2;
    let splitted = ln.split_at(midpoint);
    let first_set: HashSet<char> = splitted.0.chars().collect();
    let second_set: HashSet<char> = splitted.1.chars().collect();
    return *first_set.intersection(&second_set).next().unwrap();
}

fn find_common_c(l1: &str, l2: &str, l3: &str) -> char {
    let mut set1: HashSet<char> = l1.chars().collect();
    let set2: HashSet<char> = l2.chars().collect();
    let set3: HashSet<char> = l3.chars().collect();
    set1.retain(|e| set2.contains(e) && set3.contains(e));
    return *set1.iter().next().unwrap();
}

fn part1(file: &File) -> std::io::Result<()> {
    let mut readers = io::BufReader::new(file);
    let lines = readers.by_ref().lines();

    let mut score = 0;
    for ln in lines.flatten() {
        score += get_score(find_misplaced(&ln));
    }

    println!("{}", score);

    Ok(())
}

fn part2(file: &File) -> std::io::Result<()> {
    let mut readers = io::BufReader::new(file);
    let mut lines = readers.by_ref().lines().peekable();

    let mut score = 0;
    while lines.peek().is_some() {
        let l1 = lines.next().unwrap().unwrap();
        let l2 = lines.next().unwrap().unwrap();
        let l3 = lines.next().unwrap().unwrap();
        score += get_score(find_common_c(&l1, &l2, &l3));
    }

    println!("{}", score);

    Ok(())
}

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("examples/3");
    println!("reading path: {}", path.display());
    let mut file = File::open(path).unwrap();

    let _ = part1(&file);
    // Reset cursor to the beginning of the file
    file.seek(SeekFrom::Start(0)).unwrap();
    let _ = part2(&file);
}
