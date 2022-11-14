use std::fs;

fn main() {
    let mut input = fs::read_to_string("input.txt")
        .expect("Unable to  read file.")
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let mut raw_boards: Vec<String> = input.drain(1..).collect();

    let mut boards: Vec<Board> = Vec::new();

    while raw_boards.len() > 0 {
        let mut raw_board: Vec<String> = raw_boards.drain(..6).collect();
        raw_board.remove(0);
        boards.push(Board {
            numbers_table: raw_board
                .into_iter()
                .map(|s| s.split(' ').map(|i| i.parse::<u32>().unwrap()).collect())
                .collect(),
            truth_table: vec![vec![false; 5]; 5],
        })
    }

    let numbers: Vec<u32> = input
        .pop()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
}

struct Board {
    numbers_table: Vec<Vec<u32>>,
    truth_table: Vec<Vec<bool>>,
}

impl Board {
    fn mark(&mut self, input: u32) {
        'outer: for i in 0..5 {
            for j in 0..5 {
                if self.numbers_table[i][j] == input {
                    self.truth_table[i][j] = true;
                    break 'outer;
                }
            }
        }
    }

    fn score(&mut self) {
        // score the loop
    }
}
