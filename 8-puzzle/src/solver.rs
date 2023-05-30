use crate::board::Board;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;

pub(crate) fn solve(board: Board) -> Vec<Board> {
    let mut queue: BinaryHeap<Board> = BinaryHeap::new();

    let mut visited: HashSet<Board> = HashSet::new();

    queue.push(board.clone());

    let mut ancestors: HashMap<Board, Board> = HashMap::new();

    ancestors.insert(board.clone(), board.clone());

    while !queue.is_empty() {
        let mut now = queue.pop().unwrap();

        if now.is_goal() {
            let mut path = vec![];
            while now != board {
                path.push(now.clone());
                now = ancestors.get(&now).unwrap().clone();
            }
            path.push(board);
            path.reverse();
            return path;
        }

        visited.insert(now.clone());

        let mut neighbors = Board::nearby(&now);

        for near in neighbors.iter_mut() {
            if !visited.contains(near) {
                ancestors.insert(near.clone(), now.clone());
                near.update_from_start(&now);
                queue.push(near.clone());
            } else if now.possible() < near.dist_from_start() {
                ancestors.insert(near.clone(), now.clone());
                near.update_from_start(&now);
            }
        }
    }

    vec![]
}

pub(crate) fn manhattan(b: &Vec<Vec<usize>>) -> usize {
    let mut diff = 0;

    for i in 0..b.len() {
        for j in 0..b.len() {
            if b[i][j] != 0 {
                let row = (b[i][j] - 1) / b.len();
                let col = (b[i][j] - 1) % b.len();

                diff += max(i, row) - min(i, row) + max(j, col) - min(j, col);
            }
        }
    }

    diff
}

pub(crate) fn find_gap(b: &Vec<Vec<usize>>) -> (usize, usize) {
    for i in 0..b.len() {
        for j in 0..b.len() {
            if b[i][j] == 0 {
                return (i, j);
            }
        }
    }

    (0, 0)
}
