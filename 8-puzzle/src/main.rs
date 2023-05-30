use std::time::Instant;

use puzzle8_rust::solver::{solve, Board};

use crate::board::Board;
use crate::solver::{find_gap, manhattan, solve};

mod board;
mod solver;

fn main() {
    let data: Vec<Vec<usize>> = vec![
        vec![1, 4, 13, 11, 5],
        vec![19, 6, 14, 10, 15],
        vec![8, 16, 17, 0, 24],
        vec![9, 7, 21, 20, 12],
        vec![3, 18, 22, 23, 2],
    ];

    let gap = find_gap(&data);

    let manhattan = manhattan(&data);

    let board = Board::new(data, 0, gap, manhattan);

    let start = Instant::now();
    let solution = solve(board);
    let duration = start.elapsed();

    let mut result = String::new();

    for b in solution.iter() {
        result.push_str(b.to_string().as_str());
        result.push('\n');
    }

    result.push_str(format!("{}\n", solution.len()).as_str());

    result.push_str(format!("{} seconds", duration.as_millis() as f64 / 1000.0).as_str());

    println!("{}", result);
}
