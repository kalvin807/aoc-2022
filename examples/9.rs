use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn draw(clock: i64, x: i64) {
    let delta = x - (clock % 40) + 1;
    if delta == 0 || delta == -1 || delta == 1 {
        print!("██");
    } else {
        print!("░░");
    }

    if clock % 40 == 0 {
        print!("  {}", clock);
        println!();
    }
}

fn execute(instructions: Vec<String>) -> i64 {
    let mut strength_sum = 0;
    let mut x = 1;
    let mut clock = 0;
    let mut queue: VecDeque<(i64, i64)> = VecDeque::new();
    let mut i_iter = instructions.iter();
    let mut measure_clock = 20;
    loop {
        // cycle starts
        clock += 1;

        if queue.is_empty() {
            let ln = i_iter.next();
            match ln {
                Some(ln) => {
                    // take input
                    let mut parts = ln.split_whitespace();
                    let op = parts.next().unwrap();
                    let arg = parts.next().unwrap_or("0").parse::<i64>().unwrap();

                    // process input
                    match op {
                        "noop" => {}
                        "addx" => {
                            queue.push_back((clock + 1, arg));
                        }
                        _ => panic!("unknown op: {}", op),
                    }
                }
                _ => break,
            }
        }

        // take measure
        if clock == measure_clock {
            strength_sum += clock * x;
            // println!(
            //     "Clock: {}, X: {}, strength: {}, strength_sum: {}",
            //     clock,
            //     x,
            //     clock * x,
            //     strength_sum
            // );
            measure_clock += 40;
        }

        draw(clock, x);

        // process task queue
        while !queue.is_empty() && queue[0].0 == clock {
            let (_, v) = queue.pop_front().unwrap();
            x += v;
        }
    }
    strength_sum
}

fn runner(path: &str) -> i64 {
    println!("reading file: {}", path);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(&file);
    let instructions: Vec<String> = reader.lines().flatten().collect();

    execute(instructions)
}

fn main() {
    let path = "inputs/actual/9";
    println!("Part1: {:?}", runner(path));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let path = "inputs/test/9";
        let res = super::runner(path);
        assert_eq!(res, 13140);
    }

    // #[test]
    // fn test_part2() {
    //     let path = "inputs/test/8";
    //     let res = super::runner(path, true);
    //     assert_eq!(res, vec![19, 23, 23, 29, 26]);
    // }
}
