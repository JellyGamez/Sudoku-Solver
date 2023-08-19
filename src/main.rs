use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};
mod sudoku;
use crate::sudoku::Sudoku;


fn main() {

    let mut solved = 0;
    let mut board = [[0; 10]; 10];

    let start = Instant::now();

    //file input
    let filename = "./datasets/2.txt";
    //limit the number of grids
    let limit = 20000;

    if let Ok(file) = File::open(filename)
    {
        let lines = io::BufReader::new(file).lines();
        for (i, line) in lines.enumerate()
        {
            if i == limit
            {
                break;
            }
            if let Ok(grid) = line
            {
                for (i, digit) in grid.trim().bytes().enumerate()
                {
                    board[i / 9 + 1][i % 9 + 1] = digit as i32 - '0' as i32;
                }
                let mut sudoku: Sudoku = Sudoku::new(board);
                sudoku.bkt(0);
                solved += 1;
            }
        }
    }

    println!("Grids solved: {}", solved);
    println!("Time elapsed: {:?}", start.elapsed().as_secs_f32());
}  
