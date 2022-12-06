use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn find_start_pos(ln: &str, window_size: usize) -> u32 {
    let mut pos = window_size; // start at the end of the window
    let mut seen: VecDeque<char> = VecDeque::with_capacity(window_size);
    for c in ln.chars() {
        //* O(n), n = size of window!
        while seen.contains(&c) {
            seen.pop_front();
            pos += 1;
        }
        seen.push_back(c);
        if seen.len() == window_size {
            return pos as u32;
        }
    }
    0 // unreachable tho
}

fn runner(path: &str, is_part2: bool) -> Vec<u32> {
    println!("reading file: {}", path);
    let file = File::open(path).unwrap();
    let lines = BufReader::new(&file).lines().flatten();
    let mut res: Vec<u32> = Vec::new();
    let size = if is_part2 { 14 } else { 4 };
    for ln in lines {
        let pos = find_start_pos(&ln, size);
        res.push(pos);
    }

    res
}
fn main() {
    let path = "inputs/actual/6";
    println!("Part1: {:?}", runner(path, false));
    println!("Part2: {:?}", runner(path, true));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let path = "inputs/test/6";
        let res = super::runner(path, false);
        assert_eq!(res, vec![7, 5, 6, 10, 11]);
    }

    #[test]
    fn test_part2() {
        let path = "inputs/test/6";
        let res = super::runner(path, true);
        assert_eq!(res, vec![19, 23, 23, 29, 26]);
    }
}
