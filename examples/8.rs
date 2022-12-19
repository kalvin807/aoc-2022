use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn count_visible_so_far(row: Vec<u8>, height: i8) -> i32 {
    let mut count = 0;
    for i in (0..row.len()).rev() {
        count += 1;
        if row[i] as i8 >= height {
            break;
        }
    }
    count
}

fn count_visible_trees(grid: &Vec<Vec<u8>>) -> u64 {
    let row_count = grid.len();
    let col_count = grid[0].len();

    // Initialize boolean matrix to track visible trees
    let mut visible = vec![vec![1; col_count]; row_count];

    // Check rows from left to right
    for row in 0..row_count {
        let mut seen = vec![];
        let mut prev_height = -1;
        for col in 0..col_count {
            let height = grid[row][col] as i8;
            if height > prev_height {
                visible[row][col] *= count_visible_so_far(seen.clone(), height);
            } else {
                visible[row][col] *= 1;
            }
            seen.push(height as u8);
            prev_height = height;
        }
    }
    print_matrix(&visible);
    // Check rows from right to left
    for row in 0..row_count {
        let mut seen = vec![];
        let mut prev_height = -1;
        for col in (0..col_count).rev() {
            let height = grid[row][col] as i8;
            if height > prev_height {
                visible[row][col] *= count_visible_so_far(seen.clone(), height);
            } else {
                visible[row][col] *= 1;
            }
            seen.push(height as u8);
            prev_height = height;
        }
    }
    print_matrix(&visible);
    // Check columns from top to bottom
    for col in 0..col_count {
        let mut seen = vec![];
        let mut prev_height = -1;
        for row in 0..row_count {
            let height = grid[row][col] as i8;
            if height > prev_height {
                visible[row][col] *= count_visible_so_far(seen.clone(), height);
            } else {
                visible[row][col] *= 1;
            }
            seen.push(height as u8);
            prev_height = height;
        }
    }
    print_matrix(&visible);
    // Check columns from bottom to top
    for col in 0..col_count {
        let mut seen = vec![];
        let mut prev_height = -1;
        for row in (0..row_count).rev() {
            let height = grid[row][col] as i8;
            if height > prev_height {
                visible[row][col] *= count_visible_so_far(seen.clone(), height);
            } else {
                visible[row][col] *= 1;
            }
            seen.push(height as u8);
            prev_height = height;
        }
    }
    print_matrix(&visible);
    // Count number of true values in boolean matrix
    visible.into_iter().flatten().max().unwrap() as u64
}

fn runner(path: &str, is_part2: bool) -> u64 {
    println!("reading file: {}", path);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(&file);
    let forest = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    count_visible_trees(&forest)
}
fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        for val in row {
            print!("{} ", *val);
        }
        println!();
    }
    println!();
}

fn main() {
    let path = "inputs/actual/8";
    println!("Part1: {:?}", runner(path, false));
    println!("Part2: {:?}", runner(path, true));

    // ugly n slow... :[
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let path = "inputs/test/8";
        let res = super::runner(path, false);
        assert_eq!(res, 8);
    }

    // #[test]
    // fn test_part2() {
    //     let path = "inputs/test/8";
    //     let res = super::runner(path, true);
    //     assert_eq!(res, vec![19, 23, 23, 29, 26]);
    // }
}
