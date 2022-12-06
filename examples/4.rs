use std::fs::File;
use std::io::{BufRead, Seek, SeekFrom};
use std::{env, io};

fn get_range(ln: &str) -> (u32, u32) {
    // prob overkill for regex, split by "-"
    let mut ranges = ln.split('-');
    let a = ranges.next().unwrap().parse::<u32>().unwrap();
    let b = ranges.next().unwrap().parse::<u32>().unwrap();

    if a < b {
        return (a, b);
    }
    (b, a)
}

fn full_overlap(ln: String) -> u32 {
    let mut splitted = ln.split(',');
    let (a1, a2) = get_range(splitted.next().unwrap());
    let (b1, b2) = get_range(splitted.next().unwrap());

    let is_b_in_a = a1 <= b1 && a2 >= b2;
    let is_a_in_b = b1 <= a1 && b2 >= a2;

    match is_b_in_a || is_a_in_b {
        true => 1,
        false => 0,
    }
}

fn partial_overlap(ln: String) -> u32 {
    let mut splitted = ln.split(',');
    let (a1, a2) = get_range(splitted.next().unwrap());
    let (b1, b2) = get_range(splitted.next().unwrap());
    let some_b_in_a = (a1 <= b1 && a2 >= b1) || (a1 <= b2 && a2 >= b2);
    let some_a_in_b = (b1 <= a1 && b2 >= a1) || (b1 <= a2 && b2 >= a2);

    match some_b_in_a || some_a_in_b {
        true => 1,
        false => 0,
    }
}

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("inputs/actual/4");
    println!("reading path: {}", path.display());
    let mut file = File::open(path).unwrap();
    let lines = io::BufReader::new(&file).lines();

    let mut full_overlap_cnt: u32 = 0;
    let mut partial_overlap_cnt: u32 = 0;
    lines.for_each(|elm| {
        let ln = elm.unwrap();
        full_overlap_cnt += full_overlap(ln.clone());
        partial_overlap_cnt += partial_overlap(ln);
    });

    println!("full_overlap: {}", full_overlap_cnt);
    // Reset cursor to the beginning of the file
    file.seek(SeekFrom::Start(0)).unwrap();
    println!("partial_overlap: {}", partial_overlap_cnt);
}
