use std::io::{self, BufRead};
use std::time::Instant;
use std::fs::File;

mod sudoku;
use crate::sudoku::Sudoku;

fn main() 
{
    let mut solved = 0;
    let mut board = [[0; 10]; 10];

    let start = Instant::now();

    //file input
    let filename = "./datasets/easy.txt";
    //limit the number of grids
    let limit = 10000;

    if let Ok(file) = File::open(filename)
    {
        let lines = io::BufReader::new(file).lines();
        let lines = lines;

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
                if sudoku.solved
                {
                    solved += 1;
                }
            }
        }

        let elapsed = start.elapsed().as_secs_f32();

        println!("Grids solved: {}/{limit}", solved);
        println!("Time elapsed: {} seconds", elapsed);
        println!("Average time per grid: {} seconds", elapsed / limit as f32);
        println!("Average grids per second: {}", limit as f32 / elapsed);
    }
    else 
    {
        eprintln!("File not found.");
    }

}