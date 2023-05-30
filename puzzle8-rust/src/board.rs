use std::cmp::{max, min};
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct Board {
    data: Vec<Vec<usize>>,
    moves: usize,
    gap: (usize, usize),
    manhattan: usize,
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl Board {
    pub fn new(
        data: Vec<Vec<usize>>,
        moves: usize,
        gap: (usize, usize),
        manhattan: usize,
    ) -> Board {
        Board {
            data,
            moves,
            gap,
            manhattan,
        }
    }

    fn updated_manhattan(&self, new_gap: (usize, usize)) -> usize {
        let x = self.data[new_gap.0][new_gap.1];

        let row = (x - 1) / self.data.len();
        let col = (x - 1) % self.data.len();

        let mut manh = self.manhattan;
        manh -=
            max(new_gap.0, row) - min(new_gap.0, row) + max(new_gap.1, col) - min(new_gap.1, col);
        manh += max(self.gap.0, row) - min(self.gap.0, row) + max(self.gap.1, col)
            - min(self.gap.1, col);

        manh
    }

    fn hamming(&self) -> usize {
        let mut hamming = 0;
        let mut expected = 0;
        let max = if !self.data.is_empty() {
            self.data.len() * self.data.len() - 1
        } else {
            0
        };
        for row in self.data.iter() {
            for num in row {
                if expected < max {
                    expected += 1;
                } else {
                    expected = 0;
                }
                if *num != expected {
                    hamming += 1;
                }
            }
        }

        hamming
    }

    fn heuristics(&self) -> usize {
        self.to_end() + self.dist_from_start()
    }

    fn to_end(&self) -> usize {
        (3.0 * self.manhattan as f64) as usize
    }

    pub(crate) fn dist_from_start(&self) -> usize {
        self.moves
    }

    pub(crate) fn update_from_start(&mut self, better: &Board) {
        self.moves = better.dist_from_start() + 1
    }

    pub(crate) fn possible(&self) -> usize {
        self.dist_from_start() + 1
    }

    pub(crate) fn is_goal(&self) -> bool {
        self.hamming() == 0
    }

    pub(crate) fn nearby(current: &Board) -> Vec<Board> {
        let mut nearby = vec![];
        let gap = current.gap;
        let mut tmp = current.data.clone();

        let mut swapper = |dy: i32, dx: i32| {
            let new_gap = ((gap.0 as i32 + dy) as usize, (gap.1 as i32 + dx) as usize);
            tmp[gap.0][gap.1] = tmp[new_gap.0][new_gap.1];
            tmp[new_gap.0][new_gap.1] = 0;
            nearby.push(Board {
                data: tmp.clone(),
                moves: current.moves + 1,
                gap: new_gap,
                manhattan: current.updated_manhattan(new_gap),
            });
            tmp[new_gap.0][new_gap.1] = tmp[gap.0][gap.1];
            tmp[gap.0][gap.1] = 0;
        };

        if gap.0 != current.data.len() - 1 {
            swapper(1, 0);
        }
        if gap.0 != 0 {
            swapper(-1, 0);
        }
        if gap.1 != current.data.len() - 1 {
            swapper(0, 1);
        }
        if gap.1 != 0 {
            swapper(0, -1);
        }

        nearby
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut repr = String::new();

        for i in 0..self.data.len() {
            for j in 0..self.data.len() {
                if j != 0 {
                    repr.push(' ');
                }

                repr.push_str(&self.data[i][j].to_string());
            }

            repr.push('\n');
        }

        write!(f, "{}", repr).expect("failed to represent Board");

        Ok(())
    }
}
